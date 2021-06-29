import Head from "next/head";

function MyApp({ Component, pageProps }) {
  return (
    <>
      <Head>
        <link
          href="/fonts/JetBrainsMono-MediumItalic.ttf"
          as="font"
          crossOrigin=""
        />
      </Head>
      <Component {...pageProps} />
      <style global jsx>{`
        * {
          font-family: JetBrains Mono;
        }
      `}</style>
    </>
  );
}

export default MyApp;
