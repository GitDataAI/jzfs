import React, { useState, useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import "./layout.css"
type AuthLayoutProps = unknown;

const AuthLayout: React.FC<AuthLayoutProps> = () => {
  const [width, setWidth] = useState(window.innerWidth);

  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [height, setHeight] = useState(window.innerHeight);

  useEffect(() => {
    const handleResize = () => {
      setWidth(window.innerWidth);
      setHeight(window.innerHeight);
    };

    window.addEventListener('resize', handleResize);

    // Cleanup the event listener on component unmount
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  return (
    <div className="authLayout">
      {width > 600 ? (
        <div className="authLayout-loginWindowsLarge">
          <Outlet />
        </div>
      ) : (
        <div className="authLayout-loginWindowsSmall">
          <Outlet />
        </div>
      )}
    </div>
  );
};

export default AuthLayout;
