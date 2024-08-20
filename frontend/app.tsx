import { StrictMode } from "react";

import { Routes } from "./routes";
import "./global.css";

import type { FC } from "react";

export const App: FC = () => (
  <StrictMode>
    <Routes />
  </StrictMode>
);
