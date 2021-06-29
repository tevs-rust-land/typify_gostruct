type ActionsWrapperProps = {
  firstAction: React.ReactNode;
  secondAction: React.ReactNode;
};
export default function ActionsWrapper({
  firstAction,
  secondAction,
}: ActionsWrapperProps) {
  return (
    <div className="wrapper">
      <div className="child">{firstAction}</div>
      <div className="child">{secondAction}</div>
      <style jsx>{`
        .wrapper {
          display: flex;
          justify-content: space-between;
        }

        @media screen and (max-width: 500px) {
          .wrapper {
            align-items: center;
            flex-direction: column;
            margin-bottom: 10px;
          }
        }
      `}</style>
    </div>
  );
}
