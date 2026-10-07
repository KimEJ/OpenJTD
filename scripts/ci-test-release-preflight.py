#!/usr/bin/env python3
"""Offline command-boundary checks; never execute real Cargo/curl/git or publish."""
from pathlib import Path
import json
import os
import shutil
import subprocess
import sys
import tempfile

SCRIPT = Path(__file__).resolve().with_name('release-preflight.sh')
MOCK = r'''import json,os,sys
from pathlib import Path
scenario=json.loads(Path(os.environ['RJTD_PREFLIGHT_SCENARIO']).read_text())
tool=Path(sys.argv[0]).name;args=sys.argv[1:]
with open(os.environ['RJTD_PREFLIGHT_LOG'],'a') as out:
 out.write(json.dumps({'tool':tool,'args':args,'tokenPresent':any(os.environ.get(v) for v in ['CARGO_REGISTRY_TOKEN','CARGO_REGISTRIES_CRATES_IO_TOKEN']),'cargoHome':os.environ.get('CARGO_HOME'),'credentialsPresent':(Path(os.environ.get('CARGO_HOME','/nonexistent'))/'credentials.toml').exists()})+'\n')
if tool=='git':
 if 'rev-parse' in args:print(os.environ['RJTD_PREFLIGHT_ROOT'])
 elif 'status' in args:print(' M changed.rs' if scenario.get('dirty') else '',end='')
 else:raise SystemExit('unexpected git command')
elif tool=='cargo':
 assert not os.environ.get('CARGO_REGISTRY_TOKEN') and not os.environ.get('CARGO_REGISTRIES_CRATES_IO_TOKEN')
 assert not (Path(os.environ['CARGO_HOME'])/'credentials.toml').exists()
 assert '--locked' in args
 if args[0]=='pkgid':
  if scenario.get('pkgidFail'):raise SystemExit(7)
  name=args[args.index('-p')+1];version=scenario.get('versions',{}).get(name,'0.0.2')
  print('path+file:///fixture/'+name+'#'+(name+'@' if scenario.get('namedId') else '')+version)
 elif args[0]=='package':
  assert '--list' in args and args[args.index('--registry')+1]=='crates-io'
  if scenario.get('packageFail'):raise SystemExit(8)
  print('Cargo.toml\nLICENSE\n'+('' if scenario.get('missingReadme') else 'README.md\n')+'src/lib.rs')
 elif args[0]=='publish':
  assert '--dry-run' in args and args[args.index('--registry')+1]=='crates-io','real upload or wrong registry requested'
  if scenario.get('dryRunFail'):raise SystemExit(9)
 else:raise SystemExit('unexpected cargo command')
elif tool=='curl':
 if scenario.get('networkFail'):raise SystemExit(6)
 url=args[-1];name,version=url.rsplit('/',2)[-2:]
 if name==scenario['package']:print(scenario.get('targetStatus',404),end='')
 else:print(scenario.get('dependencyStatus',200),end='')
else:raise SystemExit('unexpected tool')
'''


def check(case, overrides=None, *, ok=True, arguments=None):
    with tempfile.TemporaryDirectory(prefix='rjtd-preflight-test-') as directory:
        root = Path(directory)
        (root/'scripts').mkdir()
        shutil.copyfile(SCRIPT, root/'scripts/release-preflight.sh')
        (root/'rjtd').mkdir()
        (root/'rjtd/Cargo.toml').write_text('[workspace]\n')
        for package in ['rjtd-core','rjtd-model','rjtd-export','rjtd-wasm','rjtd-cli','rjtd-testkit']:
            path=root/'rjtd/crates'/package;path.mkdir(parents=True)
            (path/'Cargo.toml').write_text('publish = false\n' if package=='rjtd-testkit' else 'publish = ["crates-io"]\n')
        original_home=root/'original-cargo-home';original_home.mkdir()
        (original_home/'credentials.toml').write_text('token = "fixture-only"\n')
        tools=root/'tools';tools.mkdir()
        for name in ['cargo','curl','git']:
            path=tools/name;path.write_text('#!'+sys.executable+'\n'+MOCK);path.chmod(0o755)
        scenario={'package':'rjtd-model','versions':{'rjtd-model':'0.0.2','rjtd-core':'0.0.1'}}
        scenario.update(overrides or {})
        (root/'scenario.json').write_text(json.dumps(scenario))
        log=root/'calls.jsonl'
        env=dict(os.environ,PATH=str(tools)+os.pathsep+os.environ['PATH'],CARGO_HOME=str(original_home),CARGO_REGISTRY_TOKEN='fixture-only',CARGO_REGISTRIES_CRATES_IO_TOKEN='fixture-only',RJTD_PREFLIGHT_ROOT=str(root),RJTD_PREFLIGHT_SCENARIO=str(root/'scenario.json'),RJTD_PREFLIGHT_LOG=str(log))
        args=arguments if arguments is not None else ['--package',scenario['package'],'--allow-dirty']
        result=subprocess.run(['bash',str(root/'scripts/release-preflight.sh'),*args],env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        assert (result.returncode==0)==ok,(case,result.stdout,result.stderr)
        calls=[json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []
        cargos=[row for row in calls if row['tool']=='cargo']
        assert all(not row['tokenPresent'] and not row['credentialsPresent'] and row['cargoHome']!=str(original_home) for row in cargos),case
        assert all('--dry-run' in row['args'] for row in cargos if row['args'][0]=='publish'),case
        assert all(not Path(row['cargoHome']).exists() for row in cargos),case
        assert (original_home/'credentials.toml').read_text()=='token = \"fixture-only\"\n',case
        if ok:assert any(row['args'][0]=='publish' for row in cargos),case
        else:assert not any(row['args'][0]=='publish' for row in cargos) or scenario.get('dryRunFail'),case
        urls=[row['args'][-1] for row in calls if row['tool']=='curl']
        if ok:
            target=scenario['versions'].get(scenario['package'],'0.0.2')
            assert 'https://crates.io/api/v1/crates/'+scenario['package']+'/'+target in urls,case
            assert all(url.rsplit('/',1)[-1] not in ['rjtd-core','rjtd-model'] for url in urls),case
            if scenario['package']=='rjtd-cli':
                for dependency in ['rjtd-core','rjtd-model','rjtd-export']:assert 'https://crates.io/api/v1/crates/'+dependency+'/'+scenario['versions'][dependency] in urls,case
            if scenario['package']=='rjtd-model':assert 'https://crates.io/api/v1/crates/rjtd-core/'+scenario['versions']['rjtd-core'] in urls,case
        print(case,'passed')


def main():
    check('existing name with a new version and older indexed dependency',{'namedId':True})
    check('first version and version-only Cargo package id',{'package':'rjtd-core','versions':{'rjtd-core':'0.0.1'}})
    check('CLI indexes each exact internal version',{'package':'rjtd-cli','versions':{'rjtd-cli':'0.0.3','rjtd-core':'0.0.1','rjtd-model':'0.0.2','rjtd-export':'0.0.2'}})
    check('prerelease version',{'versions':{'rjtd-model':'0.1.0-beta.2+build','rjtd-core':'0.0.1'}})
    check('published target version',{'targetStatus':200},ok=False)
    check('missing dependency version',{'dependencyStatus':404},ok=False)
    check('network failure stops before package or publish',{'networkFail':True},ok=False)
    check('unexpected target registry response',{'targetStatus':503},ok=False)
    check('unexpected dependency registry response',{'dependencyStatus':500},ok=False)
    check('Cargo version resolution failure',{'pkgidFail':True},ok=False)
    check('unsafe version rejected before registry query',{'versions':{'rjtd-model':'bad/version'}},ok=False)
    check('missing required archive file',{'missingReadme':True},ok=False)
    check('package inspection failure',{'packageFail':True},ok=False)
    check('dry-run failure remains non-uploading',{'dryRunFail':True},ok=False)
    check('dirty checkout rejected',{'dirty':True},ok=False,arguments=['--package','rjtd-model'])
    check('internal testkit rejected',{'package':'rjtd-testkit'},ok=False)
    print('Release preflight offline boundaries passed; no real Cargo/curl/git or upload executed.')


if __name__=='__main__':
    main()
