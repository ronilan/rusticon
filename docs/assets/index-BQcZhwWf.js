(async ()=>{
    (function() {
        const e = document.createElement("link").relList;
        if (e && e.supports && e.supports("modulepreload")) return;
        for (const r of document.querySelectorAll('link[rel="modulepreload"]'))n(r);
        new MutationObserver((r)=>{
            for (const c of r)if (c.type === "childList") for (const i of c.addedNodes)i.tagName === "LINK" && i.rel === "modulepreload" && n(i);
        }).observe(document, {
            childList: !0,
            subtree: !0
        });
        function t(r) {
            const c = {};
            return r.integrity && (c.integrity = r.integrity), r.referrerPolicy && (c.referrerPolicy = r.referrerPolicy), r.crossOrigin === "use-credentials" ? c.credentials = "include" : r.crossOrigin === "anonymous" ? c.credentials = "omit" : c.credentials = "same-origin", c;
        }
        function n(r) {
            if (r.ep) return;
            r.ep = !0;
            const c = t(r);
            fetch(r.href, c);
        }
    })();
    function x() {
        o.main();
    }
    function C() {
        return {
            __proto__: null,
            "./rusticon_bg.js": {
                __proto__: null,
                __wbg___wbindgen_debug_string_ab4b34d23d6778bd: function(e, t) {
                    const n = A(t), r = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, r, !0);
                },
                __wbg___wbindgen_is_function_3baa9db1a987f47d: function(e) {
                    return typeof e == "function";
                },
                __wbg___wbindgen_is_undefined_29a43b4d42920abd: function(e) {
                    return e === void 0;
                },
                __wbg___wbindgen_number_get_c7f42aed0525c451: function(e, t) {
                    const n = t, r = typeof n == "number" ? n : void 0;
                    a().setFloat64(e + 8, f(r) ? 0 : r, !0), a().setInt32(e + 0, !f(r), !0);
                },
                __wbg___wbindgen_string_get_7ed5322991caaec5: function(e, t) {
                    const n = t, r = typeof n == "string" ? n : void 0;
                    var c = f(r) ? 0 : w(r, o.__wbindgen_malloc, o.__wbindgen_realloc), i = l;
                    a().setInt32(e + 4, i, !0), a().setInt32(e + 0, c, !0);
                },
                __wbg___wbindgen_throw_6b64449b9b9ed33c: function(e, t) {
                    throw new Error(u(e, t));
                },
                __wbg__wbg_cb_unref_b46c9b5a9f08ec37: function(e) {
                    e._wbg_cb_unref();
                },
                __wbg_addEventListener_8176dab41b09531c: function() {
                    return s(function(e, t, n, r) {
                        e.addEventListener(u(t, n), r);
                    }, arguments);
                },
                __wbg_altKey_3116112ec764f316: function(e) {
                    return e.altKey;
                },
                __wbg_altKey_c4f26b40f1b826b4: function(e) {
                    return e.altKey;
                },
                __wbg_appendChild_e95c8b3b936d250a: function() {
                    return s(function(e, t) {
                        return e.appendChild(t);
                    }, arguments);
                },
                __wbg_arrayBuffer_473644614d8643a5: function(e) {
                    return e.arrayBuffer();
                },
                __wbg_body_c7b35a55457167ba: function(e) {
                    const t = e.body;
                    return f(t) ? 0 : d(t);
                },
                __wbg_button_c794bf4b1dcd7c4c: function(e) {
                    return e.button;
                },
                __wbg_buttons_9b45c5f89c8d91db: function(e) {
                    return e.buttons;
                },
                __wbg_changedTouches_6817cf10a2c671e4: function(e) {
                    return e.changedTouches;
                },
                __wbg_clearRect_5fb1d6b44e6b6738: function(e, t, n, r, c) {
                    e.clearRect(t, n, r, c);
                },
                __wbg_click_bc40376705b1e04d: function(e) {
                    e.click();
                },
                __wbg_clientX_48ead8c93aa7a872: function(e) {
                    return e.clientX;
                },
                __wbg_clientX_4b48f4fa9fb5d872: function(e) {
                    return e.clientX;
                },
                __wbg_clientY_014d3013b9b0c450: function(e) {
                    return e.clientY;
                },
                __wbg_clientY_ddcce7762c925e13: function(e) {
                    return e.clientY;
                },
                __wbg_clipboardData_f03e3b5606f47f6d: function(e) {
                    const t = e.clipboardData;
                    return f(t) ? 0 : d(t);
                },
                __wbg_clipboard_a08ffae077ba7949: function(e) {
                    return e.clipboard;
                },
                __wbg_createElement_bbd4c90086fe6f7b: function() {
                    return s(function(e, t, n) {
                        return e.createElement(u(t, n));
                    }, arguments);
                },
                __wbg_createObjectURL_46e1b0c55389893b: function() {
                    return s(function(e, t) {
                        const n = URL.createObjectURL(t), r = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                        a().setInt32(e + 4, c, !0), a().setInt32(e + 0, r, !0);
                    }, arguments);
                },
                __wbg_ctrlKey_31968cccd46bdef6: function(e) {
                    return e.ctrlKey;
                },
                __wbg_ctrlKey_a49693667722b909: function(e) {
                    return e.ctrlKey;
                },
                __wbg_dataTransfer_5fa7598d8d6c3931: function(e) {
                    const t = e.dataTransfer;
                    return f(t) ? 0 : d(t);
                },
                __wbg_deltaX_7f421a85054bc57c: function(e) {
                    return e.deltaX;
                },
                __wbg_deltaY_ca7744a8772482e1: function(e) {
                    return e.deltaY;
                },
                __wbg_detail_c850e1ba165488a1: function(e) {
                    return e.detail;
                },
                __wbg_devicePixelRatio_18e6533e6d7f4088: function(e) {
                    return e.devicePixelRatio;
                },
                __wbg_document_7a41071f2f439323: function(e) {
                    const t = e.document;
                    return f(t) ? 0 : d(t);
                },
                __wbg_drawImage_2b01d8026be6b7ab: function() {
                    return s(function(e, t, n, r, c, i, b, m, L, E) {
                        e.drawImage(t, n, r, c, i, b, m, L, E);
                    }, arguments);
                },
                __wbg_error_a6fa202b58aa1cd3: function(e, t) {
                    let n, r;
                    try {
                        n = e, r = t, console.error(u(e, t));
                    } finally{
                        o.__wbindgen_free(n, r, 1);
                    }
                },
                __wbg_files_68cba1b2e516e1ee: function(e) {
                    const t = e.files;
                    return f(t) ? 0 : d(t);
                },
                __wbg_fillRect_992c5a4646ea7a7f: function(e, t, n, r, c) {
                    e.fillRect(t, n, r, c);
                },
                __wbg_getBoundingClientRect_ddac06b2c6b52b98: function(e) {
                    return e.getBoundingClientRect();
                },
                __wbg_getComputedStyle_a23c121719ab715c: function() {
                    return s(function(e, t) {
                        const n = e.getComputedStyle(t);
                        return f(n) ? 0 : d(n);
                    }, arguments);
                },
                __wbg_getContext_fc146f8ec021d074: function() {
                    return s(function(e, t, n) {
                        const r = e.getContext(u(t, n));
                        return f(r) ? 0 : d(r);
                    }, arguments);
                },
                __wbg_getData_a20c218e8ae28672: function() {
                    return s(function(e, t, n, r) {
                        const c = t.getData(u(n, r)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        a().setInt32(e + 4, b, !0), a().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_getElementById_0b5a508c91194690: function(e, t, n) {
                    const r = e.getElementById(u(t, n));
                    return f(r) ? 0 : d(r);
                },
                __wbg_getPropertyValue_0bc8c6164d246228: function() {
                    return s(function(e, t, n, r) {
                        const c = t.getPropertyValue(u(n, r)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        a().setInt32(e + 4, b, !0), a().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_get_021cd0f12ed28063: function(e, t) {
                    const n = e[t >>> 0];
                    return f(n) ? 0 : d(n);
                },
                __wbg_get_6011fa3a58f61074: function() {
                    return s(function(e, t) {
                        return Reflect.get(e, t);
                    }, arguments);
                },
                __wbg_get_d4195ef4546b6d90: function(e, t) {
                    const n = e[t >>> 0];
                    return f(n) ? 0 : d(n);
                },
                __wbg_height_528848d067cc2221: function(e) {
                    return e.height;
                },
                __wbg_height_cc0f4b9ec7073c11: function(e) {
                    return e.height;
                },
                __wbg_innerHeight_72e7bb88c4b9ede8: function() {
                    return s(function(e) {
                        return e.innerHeight;
                    }, arguments);
                },
                __wbg_innerWidth_c7446907ab672e41: function() {
                    return s(function(e) {
                        return e.innerWidth;
                    }, arguments);
                },
                __wbg_instanceof_CanvasRenderingContext2d_24a3fe06e62b98d7: function(e) {
                    let t;
                    try {
                        t = e instanceof CanvasRenderingContext2D;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlAnchorElement_6231b5a29726bb69: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLAnchorElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlCanvasElement_ea4dfc3bb77c734b: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLCanvasElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlElement_47620edd062da622: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_Window_cc64c86c8ef9e02b: function(e) {
                    let t;
                    try {
                        t = e instanceof Window;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_key_2cbc38fa83cdb336: function(e, t) {
                    const n = t.key, r = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, r, !0);
                },
                __wbg_left_ea423c913972748d: function(e) {
                    return e.left;
                },
                __wbg_length_9f1775224cf1d815: function(e) {
                    return e.length;
                },
                __wbg_metaKey_665498d01ebfd062: function(e) {
                    return e.metaKey;
                },
                __wbg_metaKey_f8f3c1d2a5b88850: function(e) {
                    return e.metaKey;
                },
                __wbg_name_9edc86a6da7afd7a: function(e, t) {
                    const n = t.name, r = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, r, !0);
                },
                __wbg_navigator_bc077756492232c5: function(e) {
                    return e.navigator;
                },
                __wbg_new_0c7403db6e782f19: function(e) {
                    return new Uint8Array(e);
                },
                __wbg_new_227d7c05414eb861: function() {
                    return new Error;
                },
                __wbg_new_682678e2f47e32bc: function() {
                    return new Array;
                },
                __wbg_new_with_str_sequence_19880a6f9b697d54: function() {
                    return s(function(e) {
                        return new Blob(e);
                    }, arguments);
                },
                __wbg_new_with_u8_clamped_array_and_sh_fe957411824b5158: function() {
                    return s(function(e, t, n, r) {
                        return new ImageData(H(e, t), n >>> 0, r >>> 0);
                    }, arguments);
                },
                __wbg_now_a9b7df1cbee90986: function() {
                    return Date.now();
                },
                __wbg_parentNode_e94744054a57a837: function(e) {
                    const t = e.parentNode;
                    return f(t) ? 0 : d(t);
                },
                __wbg_preventDefault_f55c01cb5fd2bcc0: function(e) {
                    e.preventDefault();
                },
                __wbg_prototypesetcall_a6b02eb00b0f4ce2: function(e, t, n) {
                    Uint8Array.prototype.set.call(P(e, t), n);
                },
                __wbg_push_471a5b068a5295f6: function(e, t) {
                    return e.push(t);
                },
                __wbg_putImageData_c810e62ea70e761d: function() {
                    return s(function(e, t, n, r) {
                        e.putImageData(t, n, r);
                    }, arguments);
                },
                __wbg_querySelector_8d395ebd237ebd46: function() {
                    return s(function(e, t, n) {
                        const r = e.querySelector(u(t, n));
                        return f(r) ? 0 : d(r);
                    }, arguments);
                },
                __wbg_queueMicrotask_5d15a957e6aa920e: function(e) {
                    queueMicrotask(e);
                },
                __wbg_queueMicrotask_f8819e5ffc402f36: function(e) {
                    return e.queueMicrotask;
                },
                __wbg_removeChild_172df530ec85cc8a: function() {
                    return s(function(e, t) {
                        return e.removeChild(t);
                    }, arguments);
                },
                __wbg_remove_48cb93cf7a6c4260: function(e) {
                    e.remove();
                },
                __wbg_resolve_e6c466bc1052f16c: function(e) {
                    return Promise.resolve(e);
                },
                __wbg_revokeObjectURL_1d23b31dc4ef5f52: function() {
                    return s(function(e, t) {
                        URL.revokeObjectURL(u(e, t));
                    }, arguments);
                },
                __wbg_setAttribute_6fde4098d274155c: function() {
                    return s(function(e, t, n, r, c) {
                        e.setAttribute(u(t, n), u(r, c));
                    }, arguments);
                },
                __wbg_setProperty_0d903d23a71dfe70: function() {
                    return s(function(e, t, n, r, c) {
                        e.setProperty(u(t, n), u(r, c));
                    }, arguments);
                },
                __wbg_setTimeout_d8786dd31f90da0f: function() {
                    return s(function(e, t, n) {
                        return e.setTimeout(t, n);
                    }, arguments);
                },
                __wbg_setTransform_e43c6ac3207fe112: function() {
                    return s(function(e, t, n, r, c, i, b) {
                        e.setTransform(t, n, r, c, i, b);
                    }, arguments);
                },
                __wbg_set_download_f60111fb9c3eac2d: function(e, t, n) {
                    e.download = u(t, n);
                },
                __wbg_set_fillStyle_e51447e54357dc46: function(e, t, n) {
                    e.fillStyle = u(t, n);
                },
                __wbg_set_height_be9b2b920bd68401: function(e, t) {
                    e.height = t >>> 0;
                },
                __wbg_set_href_f253ea395272f0c4: function(e, t, n) {
                    e.href = u(t, n);
                },
                __wbg_set_id_e241e8155f198c0e: function(e, t, n) {
                    e.id = u(t, n);
                },
                __wbg_set_imageSmoothingEnabled_321379e0eb322abc: function(e, t) {
                    e.imageSmoothingEnabled = t !== 0;
                },
                __wbg_set_innerHTML_a3c82996073b31ea: function(e, t, n) {
                    e.innerHTML = u(t, n);
                },
                __wbg_set_innerText_edb4a0283df8c609: function(e, t, n) {
                    e.innerText = u(t, n);
                },
                __wbg_set_width_5cda41d4d06a14dd: function(e, t) {
                    e.width = t >>> 0;
                },
                __wbg_shiftKey_dcf8ee699c273ed2: function(e) {
                    return e.shiftKey;
                },
                __wbg_shiftKey_e483c13c966878f6: function(e) {
                    return e.shiftKey;
                },
                __wbg_stack_3b0d974bbf31e44f: function(e, t) {
                    const n = t.stack, r = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, r, !0);
                },
                __wbg_static_accessor_GLOBAL_8cfadc87a297ca02: function() {
                    const e = typeof global > "u" ? null : global;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_GLOBAL_THIS_602256ae5c8f42cf: function() {
                    const e = typeof globalThis > "u" ? null : globalThis;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_SELF_e445c1c7484aecc3: function() {
                    const e = typeof self > "u" ? null : self;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_WINDOW_f20e8576ef1e0f17: function() {
                    const e = typeof window > "u" ? null : window;
                    return f(e) ? 0 : d(e);
                },
                __wbg_style_c331a9f6564f8f62: function(e) {
                    return e.style;
                },
                __wbg_then_792e0c862b060889: function(e, t, n) {
                    return e.then(t, n);
                },
                __wbg_then_8e16ee11f05e4827: function(e, t) {
                    return e.then(t);
                },
                __wbg_top_158f7c4dd1427771: function(e) {
                    return e.top;
                },
                __wbg_touches_a66d38f0c03ba969: function(e) {
                    return e.touches;
                },
                __wbg_width_5adcb07d04d08bdf: function(e) {
                    return e.width;
                },
                __wbg_width_9673a519d7bd5a6a: function(e) {
                    return e.width;
                },
                __wbg_writeText_41e0b9b209591a06: function(e, t, n) {
                    return e.writeText(u(t, n));
                },
                __wbindgen_cast_0000000000000001: function(e, t) {
                    return g(e, t, F);
                },
                __wbindgen_cast_0000000000000002: function(e, t) {
                    return g(e, t, S);
                },
                __wbindgen_cast_0000000000000003: function(e, t) {
                    return N(e, t, M);
                },
                __wbindgen_cast_0000000000000004: function(e, t) {
                    return g(e, t, D);
                },
                __wbindgen_cast_0000000000000005: function(e, t) {
                    return g(e, t, U);
                },
                __wbindgen_cast_0000000000000006: function(e, t) {
                    return g(e, t, W);
                },
                __wbindgen_cast_0000000000000007: function(e, t) {
                    return g(e, t, K);
                },
                __wbindgen_cast_0000000000000008: function(e, t) {
                    return g(e, t, B);
                },
                __wbindgen_cast_0000000000000009: function(e, t) {
                    return g(e, t, j);
                },
                __wbindgen_cast_000000000000000a: function(e, t) {
                    return g(e, t, O);
                },
                __wbindgen_cast_000000000000000b: function(e, t) {
                    return u(e, t);
                },
                __wbindgen_init_externref_table: function() {
                    const e = o.__wbindgen_externrefs, t = e.grow(4);
                    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, !0), e.set(t + 3, !1);
                }
            }
        };
    }
    function O(_, e) {
        o.wasm_bindgen__convert__closures_____invoke__h39308e0b264aefb8(_, e);
    }
    function S(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843(_, e, t);
    }
    function M(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h04d2fce207c88e9d(_, e, t);
    }
    function D(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_3(_, e, t);
    }
    function U(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_4(_, e, t);
    }
    function W(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_5(_, e, t);
    }
    function K(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_6(_, e, t);
    }
    function B(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_7(_, e, t);
    }
    function j(_, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h003e8eaf2c484843_8(_, e, t);
    }
    function F(_, e, t) {
        const n = o.wasm_bindgen__convert__closures_____invoke__h0efcc0c1b211ef31(_, e, t);
        if (n[1]) throw X(n[0]);
    }
    function d(_) {
        const e = o.__externref_table_alloc();
        return o.__wbindgen_externrefs.set(e, _), e;
    }
    const I = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((_)=>o.__wbindgen_destroy_closure(_.a, _.b));
    function A(_) {
        const e = typeof _;
        if (e == "number" || e == "boolean" || _ == null) return `${_}`;
        if (e == "string") return `"${_}"`;
        if (e == "symbol") {
            const r = _.description;
            return r == null ? "Symbol" : `Symbol(${r})`;
        }
        if (e == "function") {
            const r = _.name;
            return typeof r == "string" && r.length > 0 ? `Function(${r})` : "Function";
        }
        if (Array.isArray(_)) {
            const r = _.length;
            let c = "[";
            r > 0 && (c += A(_[0]));
            for(let i = 1; i < r; i++)c += ", " + A(_[i]);
            return c += "]", c;
        }
        const t = /\[object ([^\]]+)\]/.exec(toString.call(_));
        let n;
        if (t && t.length > 1) n = t[1];
        else return toString.call(_);
        if (n == "Object") try {
            return "Object(" + JSON.stringify(_) + ")";
        } catch  {
            return "Object";
        }
        return _ instanceof Error ? `${_.name}: ${_.message}
${_.stack}` : n;
    }
    function P(_, e) {
        return _ = _ >>> 0, v().subarray(_ / 1, _ / 1 + e);
    }
    function H(_, e) {
        return _ = _ >>> 0, q().subarray(_ / 1, _ / 1 + e);
    }
    let y = null;
    function a() {
        return (y === null || y.buffer.detached === !0 || y.buffer.detached === void 0 && y.buffer !== o.memory.buffer) && (y = new DataView(o.memory.buffer)), y;
    }
    function u(_, e) {
        return _ = _ >>> 0, $(_, e);
    }
    let h = null;
    function v() {
        return (h === null || h.byteLength === 0) && (h = new Uint8Array(o.memory.buffer)), h;
    }
    let p = null;
    function q() {
        return (p === null || p.byteLength === 0) && (p = new Uint8ClampedArray(o.memory.buffer)), p;
    }
    function s(_, e) {
        try {
            return _.apply(this, e);
        } catch (t) {
            const n = d(t);
            o.__wbindgen_exn_store(n);
        }
    }
    function f(_) {
        return _ == null;
    }
    function N(_, e, t) {
        const n = {
            a: _,
            b: e,
            cnt: 1
        }, r = (...c)=>{
            n.cnt++;
            try {
                return t(n.a, n.b, ...c);
            } finally{
                r._wbg_cb_unref();
            }
        };
        return r._wbg_cb_unref = ()=>{
            --n.cnt === 0 && (o.__wbindgen_destroy_closure(n.a, n.b), n.a = 0, I.unregister(n));
        }, I.register(r, n, n), r;
    }
    function g(_, e, t) {
        const n = {
            a: _,
            b: e,
            cnt: 1
        }, r = (...c)=>{
            n.cnt++;
            const i = n.a;
            n.a = 0;
            try {
                return t(i, n.b, ...c);
            } finally{
                n.a = i, r._wbg_cb_unref();
            }
        };
        return r._wbg_cb_unref = ()=>{
            --n.cnt === 0 && (o.__wbindgen_destroy_closure(n.a, n.b), n.a = 0, I.unregister(n));
        }, I.register(r, n, n), r;
    }
    function w(_, e, t) {
        if (t === void 0) {
            const b = T.encode(_), m = e(b.length, 1) >>> 0;
            return v().subarray(m, m + b.length).set(b), l = b.length, m;
        }
        let n = _.length, r = e(n, 1) >>> 0;
        const c = v();
        let i = 0;
        for(; i < n; i++){
            const b = _.charCodeAt(i);
            if (b > 127) break;
            c[r + i] = b;
        }
        if (i !== n) {
            i !== 0 && (_ = _.slice(i)), r = t(r, n, n = i + _.length * 3, 1) >>> 0;
            const b = v().subarray(r + i, r + n), m = T.encodeInto(_, b);
            i += m.written, r = t(r, n, i, 1) >>> 0;
        }
        return l = i, r;
    }
    function X(_) {
        const e = o.__wbindgen_externrefs.get(_);
        return o.__externref_table_dealloc(_), e;
    }
    let k = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    k.decode();
    const Y = 2146435072;
    let R = 0;
    function $(_, e) {
        return R += e, R >= Y && (k = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), k.decode(), R = e), k.decode(v().subarray(_, _ + e));
    }
    const T = new TextEncoder;
    "encodeInto" in T || (T.encodeInto = function(_, e) {
        const t = T.encode(_);
        return e.set(t), {
            read: _.length,
            written: t.length
        };
    });
    let l = 0, o;
    function V(_, e) {
        return o = _.exports, y = null, h = null, p = null, o.__wbindgen_start(), o;
    }
    async function z(_, e) {
        if (typeof Response == "function" && _ instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming == "function") try {
                return await WebAssembly.instantiateStreaming(_, e);
            } catch (r) {
                if (_.ok && t(_.type) && _.headers.get("Content-Type") !== "application/wasm") console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", r);
                else throw r;
            }
            const n = await _.arrayBuffer();
            return await WebAssembly.instantiate(n, e);
        } else {
            const n = await WebAssembly.instantiate(_, e);
            return n instanceof WebAssembly.Instance ? {
                instance: n,
                module: _
            } : n;
        }
        function t(n) {
            switch(n){
                case "basic":
                case "cors":
                case "default":
                    return !0;
            }
            return !1;
        }
    }
    async function G(_) {
        if (o !== void 0) return o;
        _ !== void 0 && (Object.getPrototypeOf(_) === Object.prototype ? { module_or_path: _ } = _ : console.warn("using deprecated parameters for the initialization function; pass a single object instead")), _ === void 0 && (_ = new URL("" + new URL("rusticon_bg-CQB6dDTp.wasm", import.meta.url).href, import.meta.url));
        const e = C();
        (typeof _ == "string" || typeof Request == "function" && _ instanceof Request || typeof URL == "function" && _ instanceof URL) && (_ = fetch(_));
        const { instance: t, module: n } = await z(await _, e);
        return V(t);
    }
    await G();
    x();
})();
