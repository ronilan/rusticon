import { execSync, spawn } from 'child_process';

export function run(cmd) {
    console.log(`$ ${cmd}`);

    execSync(cmd, {
        stdio: 'inherit',
        shell: true
    });
}

export function spawnProcess(cmd, args) {
    return spawn(cmd, args, {
        stdio: 'inherit',
        shell: true
    });
}
