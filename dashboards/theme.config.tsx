import {DocsThemeConfig} from "nextra-theme-docs";
import {Footer} from "./components/layout/footer";
import Image from "next/image";
import {useRouter} from "next/router";
import {useConfig} from "nextra-theme-docs";

const theme: DocsThemeConfig = {
    docsRepositoryBase:
        "https://github.com/rooch-network/rooch.git",
    nextThemes: {
        defaultTheme: "light",
    },
    logo: (
        <div className="flex nx-items-center">
            <Image
                src="/logo/rooch_black_logo.png"
                alt="Rooch Architecture"
                width={20}
                height={30}
                className="dark:hidden"
            />
            <span className="nx-ml-4 font-face">Rooch Dashboard</span>
        </div>
    ),
    useNextSeoProps() {
        const {asPath} = useRouter();
        if (asPath !== "/") {
            if (asPath.includes("/docs/")) {
                return {
                    titleTemplate: "%s – Rooch Dashboard Tools",
                };
            }
            return {
                titleTemplate: "%s – Rooch Dashboard",
            };
        } else {
            return {
                titleTemplate: "%s",
            };
        }
    },
    head: function useHead() {
        const {title, frontMatter} = useConfig();
        const {asPath} = useRouter();
        const router = useRouter();
        const socialCard = "/logo/rooch-banner.png";
        const currentLang = router.locale;
        const pageDescription = frontMatter.description ? frontMatter.description : "Dashbord";
        return (
            <>
                <meta name="msapplication-TileColor" content="#ffffff"/>
                <meta name="theme-color" content="#ffffff"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <link
                    rel="alternate"
                    href={"https://rooch.network" + asPath}
                    hrefLang="x-default"
                />
                <link
                    rel="alternate"
                    href={"https://rooch.network" + asPath}
                    hrefLang="en-us"
                />
                <link
                    rel="alternate"
                    href={"https://rooch.network" + asPath}
                    hrefLang="en"
                />
                <link
                    rel="alternate"
                    href={"https://rooch.network/zh-CN" + asPath}
                    hrefLang="zh-cn"
                />
                <link
                    rel="alternate"
                    href={"https://rooch.network/zh-CN" + asPath}
                    hrefLang="zh"
                />
                <meta name="description" content={pageDescription}/>
                <meta name="og:description" content={pageDescription}/>
                <meta name="twitter:card" content="summary_large_image"/>
                <meta name="twitter:image" content={socialCard}/>
                <meta name="twitter:site:domain" content="rooch.network"/>
                <meta name="twitter:url" content="https://rooch.network"/>
                <meta name="og:image" content={socialCard}/>
                <meta name="apple-mobile-web-app-title" content="Rooch Network"/>
                <link
                    rel="icon"
                    href="/logo/rooch_black_logo.svg"
                    type="image/svg+xml"
                />
                <link rel="icon" href="/logo/rooch_black_logo.png" type="image/png"/>
                <link
                    rel="icon"
                    href="/logo/rooch_white_logo.svg"
                    type="image/svg+xml"
                    media="(prefers-color-scheme: dark)"
                />
                <link
                    rel="icon"
                    href="/logo/rooch_white_logo.png"
                    type="image/png"
                    media="(prefers-color-scheme: dark)"
                />
            </>
        );
    },
    // i18n: [
    //     {locale: "en-US", text: "English"},
    //     {locale: "zh-CN", text: "简体中文"},
    // ],
    footer: {
        component: Footer,
    },
    search: {
        component: () => undefined
    },
};

export default theme;
