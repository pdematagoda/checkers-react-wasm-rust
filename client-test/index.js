import("./node_modules/wasm-ai-thingo/wasm_ai_thingo").then((js) => {
    const initialBoard = js.generateBoard();
    const rootElement = document.getElementById('root');

    rootElement.innerHTML = 'WebAssembly with NPM';
  });
  