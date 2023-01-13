import { Outlet, Link } from "react-router-dom";


const Layout = () => {
    return (
        <>

            <Outlet />

            <div className='contentWide'>
                <span>
                    Made with 💙 and 🦀 by  <a href="https://mustak.im">Mohammad Mustakim Ali</a>
                </span>
                &nbsp;&middot; <a href="privacy-policy">Privacy</a>
                <a href="https://www.buymeacoffee.com/mustak.im" target="_blank" rel='noreferrer'>
                    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="50px" style={{ margin: "10px" }} />
                </a>
            </div>
        </>
    )
};

export default Layout;
