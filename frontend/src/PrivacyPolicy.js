
import validUrl from "valid-url"

function PrivacyPolicy() {
    return (
        <div className="App">
            <header className="App-header">
                <h1>Privacy Redirect</h1>
                <h2>Privacy Policy</h2>
            </header>

            <div className="content">
                <div className="panel" style={{ 'textAlign': 'left' }}>
                    Privacy Redirect was made for personal use first to protect my privacy. So unsurprisingly my goals were:

                    <ul>
                        <li>My browsing habit must not be logged anywhere</li>
                        <li>My privacy must be protected</li>
                        <li>I neither want to see any ad nor want to be tracked</li>
                    </ul>

                    <p>This is true for every visitors of the website and users of the service. The source code is open and anyone can examine and see how the trackers get removed without logging the actual url you are trying to visit. Instead a hash of the URL is logged that can only tell how many different URLs being processed.</p>

                    <p>Additionally, your IP address gets logged - this is necessary to prevent abuse. IPs are logged everywhere for the same reason and that is something you accept if you are using the internet. You can always protect your privacy even more by using a VPN service to hide your actual IP.</p>
                    
                    <p>Finally, there is no ad and I do not have any plans to monetize this service. It costs so little to run a service like this so event if a fraction of the people who's reading this <a href="https://www.buymeacoffee.com/mustak.im">buys me a coffee</a> this service will run forever!</p>

                    <p>Any doubt, send me an email to i at mustak.im!</p>
                    <hr/>
                    This is it! if this wasn't long enough for a privacy policy, there is the long form written by AI - which still reads much better than any typical Privacy Policies we encounter everywhere:

                    <p>Privacy Redirect was created with the primary goal of protecting personal privacy. We understand the importance of keeping browsing habits private and protecting personal information. We strive to ensure that every visitor and user of our service has a safe and secure experience on our website.</p>
                    <p>We do not log any information about your browsing habits, and we do not use any third-party tracking tools or analytics services. Additionally, we do not serve any ads on our website and have no plans to monetize the service.</p>
                    <p>To ensure transparency, our source code is open for examination, and anyone can review how we remove trackers without logging the actual URL you are trying to visit. Instead, we log a hash of the URL, which can only indicate the number of different URLs being processed.</p>
                    <p>We do log IP addresses to prevent abuse, as this is a common practice on the internet. However, you can always take additional steps to protect your privacy by using a VPN service to conceal your IP address.</p>
                    <p>We believe that privacy is a fundamental right and are committed to maintaining the highest standards of privacy protection. Our service is low-cost to run and we appreciate any support, even a small donation, to keep this service running for all users</p>
                </div>
            </div>

        </div>
    );
}

export default PrivacyPolicy;
