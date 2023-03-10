// ==UserScript==
// @name         防偷窥神器
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the world!
// @author       You
// @match        *https://juejin.cn/notification/im?participantId*
// @icon         https://lf3-cdn-tos.bytescm.com/obj/static/xitu_juejin_web/e08da34488b114bd4c665ba2fa520a31.svg
// @grant        none
// ==/UserScript==

(function () {
  'use strict';


  // 页面非活跃状态时，整个模糊处理



  function addStylesheetRules(rules) {
    const styleEl = document.createElement("style");

    // Append <style> element to <head>
    document.head.appendChild(styleEl);

    // Grab style element's sheet
    const styleSheet = styleEl.sheet;

    for (let i = 0; i < rules.length; i++) {
      let j = 1,
        rule = rules[i],
        selector = rule[0],
        propStr = "";
      // If the second argument of a rule is an array of arrays, correct our variables.
      if (Array.isArray(rule[1][0])) {
        rule = rule[1];
        j = 0;
      }

      for (let pl = rule.length; j < pl; j++) {
        const prop = rule[j];
        propStr += `${prop[0]}: ${prop[1]}${prop[2] ? " !important" : ""};\n`;
      }

      // Insert CSS Rule
      styleSheet.insertRule(
        `${selector}{${propStr}}`,
        styleSheet.cssRules.length
      );
    }
  }


  function enterSecret() {
    addStylesheetRules([['*', ['filter', 'blur(10px)']]]);
    addStylesheetRules([['*:hover', ['filter', 'none']]]);
    button.style.display = 'none';
  }

  const button = document.createElement('button');
  button.style.position = 'fixed';
  button.style.bottom = '10px';
  button.style.left = '10px'
  button.style.padding = '6px 12px'
  button.innerText = '进入隐藏模式'
  button.style.zIndex = '9999'

  document.body.appendChild(button);
  button.onclick = enterSecret;
  // Your code here...
})();
