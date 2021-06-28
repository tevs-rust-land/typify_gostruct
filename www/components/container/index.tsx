export default function Container({ children }) {
  return (
    <div className="container">
      {children}
      <style jsx>{`
        .container {
          width: 100%;
          margin: auto;
          max-width: 900px;
        }
      `}</style>
    </div>
  );
}
