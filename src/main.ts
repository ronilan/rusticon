import init, { main } from "../pkg/rusticon.js";

async function go() {
  await init();
  main();
}

go();
