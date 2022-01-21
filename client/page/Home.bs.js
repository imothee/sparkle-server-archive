// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Curry from "rescript/lib/es6/curry.js";
import * as React from "react";
import * as Models from "../shared/Models.bs.js";
import * as Belt_Array from "rescript/lib/es6/belt_Array.js";

function Home(Props) {
  var match = React.useState(function () {
        return [];
      });
  var setApps = match[1];
  var getApps = function (param) {
    fetch("/api/apps").then(function (prim) {
            return prim.json();
          }).then(function (data) {
          var response = Models.Decode.appsResponse(data);
          Curry._1(setApps, (function (_prev) {
                  return response.apps;
                }));
          return Promise.resolve(undefined);
        });
    
  };
  React.useEffect((function () {
          getApps(undefined);
          
        }), []);
  var appRows = Belt_Array.map(match[0], (function (app) {
          return React.createElement("tr", {
                      key: String(app.id),
                      className: "hover:bg-gray-100"
                    }, React.createElement("td", {
                          className: "p-4 whitespace-nowrap text-base text-gray-900"
                        }, app.name), React.createElement("td", {
                          className: "p-4 whitespace-nowrap text-base text-gray-900"
                        }, app.slug), React.createElement("td", {
                          className: "p-4 whitespace-nowrap text-base text-gray-900"
                        }, app.description), React.createElement("td", {
                          className: "p-4 whitespace-nowrap text-base text-gray-900"
                        }, React.createElement("a", {
                              className: "text-brand",
                              href: "/admin/apps/" + String(app.id) + "/metrics"
                            }, "View Metrics")), React.createElement("td", {
                          className: "p-4 whitespace-nowrap space-x-2"
                        }, React.createElement("button", {
                              className: "text-white bg-cyan-600 hover:bg-cyan-700 focus:ring-4 focus:ring-cyan-200 font-medium rounded-lg text-sm inline-flex items-center px-3 py-2 text-center"
                            }, React.createElement("svg", {
                                  className: "mr-2 h-5 w-5",
                                  fill: "currentColor",
                                  viewBox: "0 0 20 20",
                                  xmlns: "http://www.w3.org/2000/svg"
                                }, React.createElement("path", {
                                      d: "M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z"
                                    }), React.createElement("path", {
                                      clipRule: "evenodd",
                                      d: "M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z",
                                      fillRule: "evenodd"
                                    })), "Edit App")));
        }));
  return React.createElement("div", {
              className: "max-w-[75rem] mx-auto px-4 sm:px-6 lg:px-8 py-20"
            }, React.createElement("div", {
                  className: "p-4 bg-white block sm:flex items-center justify-between border-b border-gray-200 lg:mt-1.5"
                }, React.createElement("div", {
                      className: "mb-1 w-full"
                    }, React.createElement("h1", {
                          className: "text-xl sm:text-2xl font-semibold text-gray-900"
                        }, "Apps"))), React.createElement("div", {
                  className: "flex flex-col"
                }, React.createElement("table", {
                      className: "table-fixed min-w-full divide-y divide-gray-200"
                    }, React.createElement("thead", {
                          className: "bg-light-accent"
                        }, React.createElement("tr", undefined, React.createElement("th", {
                                  className: "p-4 text-left text-xs font-medium text-dark-shade",
                                  scope: "col"
                                }, "Name"), React.createElement("th", {
                                  className: "p-4 text-left text-xs font-medium text-dark-shade",
                                  scope: "col"
                                }, "Slug"), React.createElement("th", {
                                  className: "p-4 text-left text-xs font-medium text-dark-shade",
                                  scope: "col"
                                }, "Description"), React.createElement("th", {
                                  className: "p-4",
                                  scope: "col"
                                }), React.createElement("th", {
                                  className: "p-4",
                                  scope: "col"
                                }))), React.createElement("tbody", {
                          className: "bg-white divide-y divide-gray-200"
                        }, appRows))));
}

var make = Home;

export {
  make ,
  
}
/* react Not a pure module */
