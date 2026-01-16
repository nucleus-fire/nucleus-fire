import{o as y,p as S,q as h,t as m,r as i,_ as f,n as e,M as j,L as w,O as g,S as M}from"./components-BQfbt_kk.js";/**
 * @remix-run/react v2.17.2
 *
 * Copyright (c) Remix Software Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE.md file in the root directory of this source tree.
 *
 * @license MIT
 */let a="positions";function k({getKey:r,...l}){let{isSpaMode:u}=y(),o=S(),c=h();m({getKey:r,storageKey:a});let p=i.useMemo(()=>{if(!r)return null;let t=r(o,c);return t!==o.key?t:null},[]);if(u)return null;let d=((t,x)=>{if(!window.history.state||!window.history.state.key){let s=Math.random().toString(32).slice(2);window.history.replaceState({key:s},"")}try{let n=JSON.parse(sessionStorage.getItem(t)||"{}")[x||window.history.state.key];typeof n=="number"&&window.scrollTo(0,n)}catch(s){console.error(s),sessionStorage.removeItem(t)}}).toString();return i.createElement("script",f({},l,{suppressHydrationWarning:!0,dangerouslySetInnerHTML:{__html:`(${d})(${JSON.stringify(a)}, ${JSON.stringify(p)})`}}))}function L(){return e.jsxs("html",{lang:"en",children:[e.jsxs("head",{children:[e.jsx("meta",{charSet:"utf-8"}),e.jsx(j,{}),e.jsx(w,{})]}),e.jsxs("body",{children:[e.jsx(g,{}),e.jsx(k,{}),e.jsx(M,{})]})]})}export{L as default};
