import { f } from 'rolldown-wasm-issue-wasm';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <div id="result"></div>
  </div>
`

try {
  const value = f({
    f: () => { },
    v: 1
  });
  document.querySelector<HTMLDivElement>('#result')!.textContent = `get_v returned: ${value}`;
} catch (error) {
  document.querySelector<HTMLDivElement>('#result')!.textContent = `failed: ${error}`;
}
