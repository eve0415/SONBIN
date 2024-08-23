import { Switch, Route } from "wouter";

import { Home } from "./pages/home";

import type { FC } from "react";

export const Routes: FC = () => (
  <Switch>
    <Route component={Home} path="/" />
    <Route>404, Not Found</Route>
  </Switch>
);
