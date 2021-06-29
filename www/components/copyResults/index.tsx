type CopyToClipBoardProps = {
  result: string;
};

export default function CopyResults(props: CopyToClipBoardProps) {
  const copyText = () => {
    if (!navigator.clipboard) {
      return alert(
        "The clipboard API isnt available in your browser, please copy from the editor box directly."
      );
    }

    return navigator.clipboard
      .writeText(props.result)
      .then(() => {
        alert("copied to clipboard");
      })
      .catch((err) => {
        alert(err);
      });
  };
  return (
    <>
      <button className="copy-results" onClick={copyText}>
        Copy Results
      </button>
      <style jsx>
        {`
          .copy-results {
            background-color: #fff;
            border: solid 1px;
            color: black;
            padding: 10px 10px;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            font-size: 16px;
            border-radius: 5px;
          }
        `}
      </style>
    </>
  );
}
