cd pkg
npm link

cd ../client-test
npm link wasm-ai-thingo

npm install
npm run serve