import './App.css';
import { useEffect, useState } from "react"
import { cleanUrl, SERVER } from './http';
import validUrl, { isHttpUri } from "valid-url"

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
        <p>
          Use this service to remove all known tracker and hide your referrer before redirecting your visitor to another site.
        </p>
        <p>
          Simply prefix the url with '<code>{SERVER}/?</code>'.
        </p>
        <p>
        Paste a link below to see preview ⚡
        </p>
        <input type="text" defaultValue={state.url} onChange={updateResult} placeholder="Paste an URL" />

        <form>
          <div style={{ fontSize: 'x-large' }}>
            <p>
              <a href={SERVER + '/?' + state.url} target="_blank" className='previewLink'>
                <span id="host">{SERVER}/</span>?<span id="orgUrl">{state.url}</span>
              </a>
            </p>
          </div>
          Redirects to:
          <br/><br/>
          <input type="text" value={state.cleaned} placeholder="Cleaned URL will appear hear" readOnly={true} />
        </form>
        <p>
          If you specify <code>Content-Type: application/json</code> then you get a json response.
        </p>
        <pre className='preview'>{ state.json }</pre>
      </div >
    </div >
  );
}

export default App;
