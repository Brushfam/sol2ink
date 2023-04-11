"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[57],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>h});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),p=c(r),m=o,h=p["".concat(s,".").concat(m)]||p[m]||d[m]||a;return r?n.createElement(h,i(i({ref:t},u),{},{components:r})):n.createElement(h,i({ref:t},u))}));function h(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=m;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[p]="string"==typeof e?e:o,i[1]=l;for(var c=2;c<a;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},5436:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>l,toc:()=>c});var n=r(7462),o=(r(7294),r(3905));const a={sidebar_position:7,title:"Assembling a contract"},i=void 0,l={unversionedId:"how_it_works/assembler",id:"how_it_works/assembler",title:"Assembling a contract",description:"Sol2Ink has everything it needs; now, it needs to mix it. Here we will clarify what may not be obvious.",source:"@site/docs/how_it_works/assembler.md",sourceDirName:"how_it_works",slug:"/how_it_works/assembler",permalink:"/sol2ink/how_it_works/assembler",draft:!1,editUrl:"https://github.com/Brushfam/sol2ink/tree/main/docs/docs/how_it_works/assembler.md",tags:[],version:"current",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"Assembling a contract"},sidebar:"tutorialSidebar",previous:{title:"Parsing expressions",permalink:"/sol2ink/how_it_works/parsing_expressions"},next:{title:"Known issues",permalink:"/sol2ink/issues"}},s={},c=[{value:"Error",id:"error",level:3},{value:"Storage",id:"storage",level:3}],u={toc:c},p="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"Sol2Ink has everything it needs; now, it needs to mix it. Here we will clarify what may not be obvious."),(0,o.kt)("h3",{id:"error"},"Error"),(0,o.kt)("p",null,"Each contract and library will contain the following error definition: "),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#[derive(Debug, Encode, Decode, PartialEq)]\n#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]\npub enum Error {\n    Custom(String),\n}\n')),(0,o.kt)("p",null,"This error will be used as the error type when returning results from the contract functions. In the future versions, we plan on creating an Error enum variant for each error a contract can produce. So instead of ",(0,o.kt)("inlineCode",{parentName:"p"},'Err(Error::Custom(String::from("No allowance")))')," Sol2Ink will produce ",(0,o.kt)("inlineCode",{parentName:"p"},"Err(Error::NoAllowance)"),"."),(0,o.kt)("h3",{id:"storage"},"Storage"),(0,o.kt)("p",null,"Openbrush simplifies the work with storage and allows the upgradeability of the storage; that is why we use the following approach. This approach will also streamline our future development when our contract uses multiple traits, etc. For now, we define a storage key inside the contract, the state variables in a struct which will use this storage key, and this struct itself is the member of the contract storage. The whole storage will look something like this:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);\n\n#[derive(Default, Debug)]\n#[openbrush::upgradeable_storage(STORAGE_KEY)]\npub struct Data {\n    pub value: u128,\n}\n\n#[ink(storage)]\n#[derive(Default, SpreadAllocate, Storage)]\npub struct Contract {\n    #[storage_field]\n    data: Data,\n}\n")),(0,o.kt)("p",null,"Accessing the ",(0,o.kt)("inlineCode",{parentName:"p"},"value")," state variables inside the contract looks like ",(0,o.kt)("inlineCode",{parentName:"p"},"self.data.value"),". "),(0,o.kt)("p",null,"Sol2Ink will generate the functions of the contract inside the impl section. Note the following:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"the constructor will be called new and will have the ",(0,o.kt)("inlineCode",{parentName:"li"},"#[ink(constructor)]")," attribute"),(0,o.kt)("li",{parentName:"ul"},"the constructor will be generated even if it is empty or does not exist in the original contract"),(0,o.kt)("li",{parentName:"ul"},"public/external messages will have the ",(0,o.kt)("inlineCode",{parentName:"li"},"#[ink(message)]")," attribute"),(0,o.kt)("li",{parentName:"ul"},"private/internal functions will be prefixed with ",(0,o.kt)("inlineCode",{parentName:"li"},"_"))))}d.isMDXComponent=!0}}]);