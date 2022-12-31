import './App.css';
import { useEffect, useState } from "react"
import { cleanUrl, SERVER } from './http';
import validUrl from "valid-url"
import getForFirefox from './get-the-addon-fx-apr-2020.svg'

function App() {
  const [state, setState] = useState({
    url: "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5EdUmBgUY",
    cleaned: "",
    json: ""
  });

  function updateResult(event) {
    const url = event.target.value;
    setState(p => {
      return {
        ...p,
        url: url
      };
    });

    if (validUrl.isUri(url)) {
      cleanUrl(url).then(json => setState(p => {
        return {
          ...p,
          url: url,
          cleaned: json.cleaned_url,
          json: JSON.stringify(json, null, 5)
        };
      }));
    }
  }

  useEffect(() => {
    updateResult({ target: { value: state.url } });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <h1>Privacy Redirect</h1>
        <a href="https://github.com/mustakimali/privacy-redirect">Github</a>
      </header>

      <div className="content">
        <div className="panel">
          <p>
            Removes all known trackers and hide your referrer before redirecting your visitor to another site.
          </p>
          <div>
            <a href="https://addons.mozilla.org/en-US/firefox/addon/privacydir/">
              <img src={getForFirefox} width="150px" />
            </a>
          </div>

          <div style={{ fontSize: "small" }}>
            <a href="#for-your-website">Installation instruction for own your website</a>
          </div>

          <h2>How does it work?</h2>
          <p>
            Simply prefix the link with <code>{SERVER}/?</code>
          </p>
          <p>
            Paste a link below to see preview ⚡
          </p>
          <input type="text" defaultValue={state.url} onChange={updateResult} placeholder="Paste a link" />

          <form>
            <div style={{ fontSize: 'x-large' }}>
              <p>
                <a href={SERVER + '/?' + state.url} target="_blank" className='previewLink'>
                  <span id="host">{SERVER}/</span>?<span id="orgUrl">{state.url}</span>
                </a>
              </p>
            </div>
            Redirects to cleanded link by <a href="https://whatsmyreferer.com/?utm_source=privacy-redirect">hiding your referrer</a>:
            <br /><br />
            <input type="text" value={state.cleaned} placeholder="Cleaned link will appear hear" readOnly={true} />
          </form>

          <div>
            <small>
              If you specify <code>Content-Type: application/json</code> then you get a json response.
            </small>

            <pre className='preview'>{state.json}</pre>
          </div>

        </div >

        <div className="panel">
          <h2 id="for-your-website">For your website</h2>

          <p>Add the following script in your website</p>
          <code>
            &lt;script src="{SERVER}/app/script.js"&gt;&lt;/script&gt;
          </code>
        </div>
      </div >

      <div className='contentWide'>
        <span>
          Made with 💙 and 🦀 by  <a href="https://mustak.im">Mohammad Mustakim Ali</a>
        </span>
        <a href="https://www.buymeacoffee.com/mustak.im" target="_blank">
          <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="50px" style={{ margin: "10px" }} />
        </a>
      </div>
    </div>
  );
}

export default App;
