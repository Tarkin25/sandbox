import App, { AppInitialProps, AppContext } from "next/app";
import { wrapper } from "../app/store";

class WrappedApp extends App<AppInitialProps> {

    public static getInitialProps = (context: any) =>
    wrapper.getInitialAppProps(store => async ({Component, ctx}: AppContext) => {
        return {
            pageProps: {
                // Call page-level getInitialProps
                ...(Component.getInitialProps ? await Component.getInitialProps({...ctx, store}) : {})
            }
        };
    })(context);

    public render() {
        const {Component, pageProps} = this.props;

        return (
            <Component {...pageProps} />
        )
    }

}

export default wrapper.withRedux(WrappedApp);