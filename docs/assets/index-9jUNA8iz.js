(async ()=>{
    (function() {
        const e = document.createElement("link").relList;
        if (e && e.supports && e.supports("modulepreload")) return;
        for (const _ of document.querySelectorAll('link[rel="modulepreload"]'))n(_);
        new MutationObserver((_)=>{
            for (const c of _)if (c.type === "childList") for (const i of c.addedNodes)i.tagName === "LINK" && i.rel === "modulepreload" && n(i);
        }).observe(document, {
            childList: !0,
            subtree: !0
        });
        function t(_) {
            const c = {};
            return _.integrity && (c.integrity = _.integrity), _.referrerPolicy && (c.referrerPolicy = _.referrerPolicy), _.crossOrigin === "use-credentials" ? c.credentials = "include" : _.crossOrigin === "anonymous" ? c.credentials = "omit" : c.credentials = "same-origin", c;
        }
        function n(_) {
            if (_.ep) return;
            _.ep = !0;
            const c = t(_);
            fetch(_.href, c);
        }
    })();
    function C() {
        o.main();
    }
    function S() {
        return {
            __proto__: null,
            "./rusticon_bg.js": {
                __proto__: null,
                __wbg___wbindgen_debug_string_07cb72cfcc952e2b: function(e, t) {
                    const n = x(t), _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg___wbindgen_is_function_2f0fd7ceb86e64c5: function(e) {
                    return typeof e == "function";
                },
                __wbg___wbindgen_is_undefined_244a92c34d3b6ec0: function(e) {
                    return e === void 0;
                },
                __wbg___wbindgen_jsval_eq_403eaa3610500a25: function(e, t) {
                    return e === t;
                },
                __wbg___wbindgen_number_get_dd6d69a6079f26f1: function(e, t) {
                    const n = t, _ = typeof n == "number" ? n : void 0;
                    s().setFloat64(e + 8, a(_) ? 0 : _, !0), s().setInt32(e + 0, !a(_), !0);
                },
                __wbg___wbindgen_string_get_965592073e5d848c: function(e, t) {
                    const n = t, _ = typeof n == "string" ? n : void 0;
                    var c = a(_) ? 0 : w(_, o.__wbindgen_malloc, o.__wbindgen_realloc), i = l;
                    s().setInt32(e + 4, i, !0), s().setInt32(e + 0, c, !0);
                },
                __wbg___wbindgen_throw_9c75d47bf9e7731e: function(e, t) {
                    throw new Error(f(e, t));
                },
                __wbg__wbg_cb_unref_158e43e869788cdc: function(e) {
                    e._wbg_cb_unref();
                },
                __wbg_addEventListener_a95e75babfc4f5a3: function() {
                    return u(function(e, t, n, _) {
                        e.addEventListener(f(t, n), _);
                    }, arguments);
                },
                __wbg_altKey_0a7b13357fc7557d: function(e) {
                    return e.altKey;
                },
                __wbg_altKey_6c67d807c153b5b3: function(e) {
                    return e.altKey;
                },
                __wbg_appendChild_f8e0d8251588e3d1: function() {
                    return u(function(e, t) {
                        return e.appendChild(t);
                    }, arguments);
                },
                __wbg_arrayBuffer_43b2ec162168e6e1: function(e) {
                    return e.arrayBuffer();
                },
                __wbg_body_9a319c5d4ea2d0d8: function(e) {
                    const t = e.body;
                    return a(t) ? 0 : d(t);
                },
                __wbg_button_9121eff76035e6f3: function(e) {
                    return e.button;
                },
                __wbg_buttons_6d1f718b1b841b35: function(e) {
                    return e.buttons;
                },
                __wbg_call_add9e5a76382e668: function() {
                    return u(function(e, t) {
                        return e.call(t);
                    }, arguments);
                },
                __wbg_changedTouches_c07bba8749d1210a: function(e) {
                    return e.changedTouches;
                },
                __wbg_clearRect_4c8837d514ced7c2: function(e, t, n, _, c) {
                    e.clearRect(t, n, _, c);
                },
                __wbg_clientX_d68312e38d37c06a: function(e) {
                    return e.clientX;
                },
                __wbg_clientX_f4fc7597502cac2b: function(e) {
                    return e.clientX;
                },
                __wbg_clientY_43c6b0f950541803: function(e) {
                    return e.clientY;
                },
                __wbg_clientY_c414f2d35e1ec005: function(e) {
                    return e.clientY;
                },
                __wbg_clipboardData_1651ed0a9e8a1d12: function(e) {
                    const t = e.clipboardData;
                    return a(t) ? 0 : d(t);
                },
                __wbg_clipboard_ed0015a88db5242e: function(e) {
                    return e.clipboard;
                },
                __wbg_close_e7e25c838eb721c7: function(e) {
                    return e.close();
                },
                __wbg_createElement_679cad83bb50288c: function() {
                    return u(function(e, t, n) {
                        return e.createElement(f(t, n));
                    }, arguments);
                },
                __wbg_createWritable_4bf12fbc2aa85c70: function(e) {
                    return e.createWritable();
                },
                __wbg_ctrlKey_68f7b8620ddfccc8: function(e) {
                    return e.ctrlKey;
                },
                __wbg_ctrlKey_7b559591aa96b86e: function(e) {
                    return e.ctrlKey;
                },
                __wbg_dataTransfer_3bc7d1c3f538bec3: function(e) {
                    const t = e.dataTransfer;
                    return a(t) ? 0 : d(t);
                },
                __wbg_deltaX_aacd03436b6f8a73: function(e) {
                    return e.deltaX;
                },
                __wbg_deltaY_02a7c4ae29ceeff0: function(e) {
                    return e.deltaY;
                },
                __wbg_detail_cf73a8e83d9c76ac: function(e) {
                    return e.detail;
                },
                __wbg_devicePixelRatio_3a60c85ae6458d68: function(e) {
                    return e.devicePixelRatio;
                },
                __wbg_document_69bb6a2f7927d532: function(e) {
                    const t = e.document;
                    return a(t) ? 0 : d(t);
                },
                __wbg_drawImage_c9a18ca8d3968245: function() {
                    return u(function(e, t, n, _, c, i, b, m, A, R) {
                        e.drawImage(t, n, _, c, i, b, m, A, R);
                    }, arguments);
                },
                __wbg_error_a6fa202b58aa1cd3: function(e, t) {
                    let n, _;
                    try {
                        n = e, _ = t, console.error(f(e, t));
                    } finally{
                        o.__wbindgen_free(n, _, 1);
                    }
                },
                __wbg_fillRect_9219f775d7e8e73e: function(e, t, n, _, c) {
                    e.fillRect(t, n, _, c);
                },
                __wbg_getBoundingClientRect_e0fb035288f4a416: function(e) {
                    return e.getBoundingClientRect();
                },
                __wbg_getComputedStyle_041ecb5b5cae0ab8: function() {
                    return u(function(e, t) {
                        const n = e.getComputedStyle(t);
                        return a(n) ? 0 : d(n);
                    }, arguments);
                },
                __wbg_getContext_f17252002286474d: function() {
                    return u(function(e, t, n) {
                        const _ = e.getContext(f(t, n));
                        return a(_) ? 0 : d(_);
                    }, arguments);
                },
                __wbg_getData_9557835f7d36eb7f: function() {
                    return u(function(e, t, n, _) {
                        const c = t.getData(f(n, _)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        s().setInt32(e + 4, b, !0), s().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_getElementById_22becc83cca95cc2: function(e, t, n) {
                    const _ = e.getElementById(f(t, n));
                    return a(_) ? 0 : d(_);
                },
                __wbg_getFile_b0cfc8b459455437: function(e) {
                    return e.getFile();
                },
                __wbg_getPropertyValue_feecd512625819d9: function() {
                    return u(function(e, t, n, _) {
                        const c = t.getPropertyValue(f(n, _)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        s().setInt32(e + 4, b, !0), s().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_get_07917391f924c996: function(e, t) {
                    const n = e[t >>> 0];
                    return a(n) ? 0 : d(n);
                },
                __wbg_get_40e0fec8ba97ee29: function(e, t) {
                    const n = e[t >>> 0];
                    return a(n) ? 0 : d(n);
                },
                __wbg_get_41476db20fef99a8: function() {
                    return u(function(e, t) {
                        return Reflect.get(e, t);
                    }, arguments);
                },
                __wbg_height_74c12c942761f846: function(e) {
                    return e.height;
                },
                __wbg_height_f036cb27636625f6: function(e) {
                    return e.height;
                },
                __wbg_innerHeight_c14a4766311600aa: function() {
                    return u(function(e) {
                        return e.innerHeight;
                    }, arguments);
                },
                __wbg_innerWidth_7c4aebd38eae8a77: function() {
                    return u(function(e) {
                        return e.innerWidth;
                    }, arguments);
                },
                __wbg_instanceof_CanvasRenderingContext2d_b433938013de3a1e: function(e) {
                    let t;
                    try {
                        t = e instanceof CanvasRenderingContext2D;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlCanvasElement_0ac74d5643067956: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLCanvasElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlElement_ca58d4b8fb43f464: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_Window_4153c1818a1c0c0b: function(e) {
                    let t;
                    try {
                        t = e instanceof Window;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_items_a888bccb93bb32b4: function(e) {
                    return e.items;
                },
                __wbg_key_2e79b9dbd4550ab3: function(e, t) {
                    const n = t.key, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg_kind_5ec84925f26f55b7: function(e, t) {
                    const n = t.kind, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg_left_ed21748ed5f587d7: function(e) {
                    return e.left;
                },
                __wbg_length_2d7acd34450594cf: function(e) {
                    return e.length;
                },
                __wbg_length_ba3c032602efe310: function(e) {
                    return e.length;
                },
                __wbg_metaKey_ef659f8598121617: function(e) {
                    return e.metaKey;
                },
                __wbg_metaKey_f8e5beafe081f6d6: function(e) {
                    return e.metaKey;
                },
                __wbg_name_5c12bb39167062e4: function(e, t) {
                    const n = t.name, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg_name_d6396501ec1b2634: function(e, t) {
                    const n = t.name, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg_navigator_f3468c6dc9006b7c: function(e) {
                    return e.navigator;
                },
                __wbg_new_227d7c05414eb861: function() {
                    return new Error;
                },
                __wbg_new_8454eee672b2ba6e: function(e) {
                    return new Uint8Array(e);
                },
                __wbg_new_with_u8_clamped_array_and_sh_a4ac3311668de769: function() {
                    return u(function(e, t, n, _) {
                        return new ImageData(q(e, t), n >>> 0, _ >>> 0);
                    }, arguments);
                },
                __wbg_now_4f457f10f864aec5: function() {
                    return Date.now();
                },
                __wbg_parentNode_c5865dc42e23bdcd: function(e) {
                    const t = e.parentNode;
                    return a(t) ? 0 : d(t);
                },
                __wbg_preventDefault_2c34c219d9b04b86: function(e) {
                    e.preventDefault();
                },
                __wbg_prototypesetcall_fd4050e806e1d519: function(e, t, n) {
                    Uint8Array.prototype.set.call(j(e, t), n);
                },
                __wbg_putImageData_3c24c64a03f8b92f: function() {
                    return u(function(e, t, n, _) {
                        e.putImageData(t, n, _);
                    }, arguments);
                },
                __wbg_querySelector_a3b1f840e2672b49: function() {
                    return u(function(e, t, n) {
                        const _ = e.querySelector(f(t, n));
                        return a(_) ? 0 : d(_);
                    }, arguments);
                },
                __wbg_queueMicrotask_40ac6ffc2848ba77: function(e) {
                    queueMicrotask(e);
                },
                __wbg_queueMicrotask_74d092439f6494c1: function(e) {
                    return e.queueMicrotask;
                },
                __wbg_removeChild_e2533909b124fb03: function() {
                    return u(function(e, t) {
                        return e.removeChild(t);
                    }, arguments);
                },
                __wbg_resolve_9feb5d906ca62419: function(e) {
                    return Promise.resolve(e);
                },
                __wbg_setAttribute_50dcf32d70e1628c: function() {
                    return u(function(e, t, n, _, c) {
                        e.setAttribute(f(t, n), f(_, c));
                    }, arguments);
                },
                __wbg_setProperty_d6673329a267577b: function() {
                    return u(function(e, t, n, _, c) {
                        e.setProperty(f(t, n), f(_, c));
                    }, arguments);
                },
                __wbg_setTimeout_d007c6f72100a5e1: function() {
                    return u(function(e, t, n) {
                        return e.setTimeout(t, n);
                    }, arguments);
                },
                __wbg_setTransform_f25014a0bb3cb050: function() {
                    return u(function(e, t, n, _, c, i, b) {
                        e.setTransform(t, n, _, c, i, b);
                    }, arguments);
                },
                __wbg_set_fillStyle_a3656c7c5d4ad803: function(e, t, n) {
                    e.fillStyle = f(t, n);
                },
                __wbg_set_height_89a4ecd0f9cc3dfa: function(e, t) {
                    e.height = t >>> 0;
                },
                __wbg_set_id_f1257005d3691e07: function(e, t, n) {
                    e.id = f(t, n);
                },
                __wbg_set_imageSmoothingEnabled_4bc0c7e39aa4d5f5: function(e, t) {
                    e.imageSmoothingEnabled = t !== 0;
                },
                __wbg_set_innerHTML_faa6730a8fd54513: function(e, t, n) {
                    e.innerHTML = f(t, n);
                },
                __wbg_set_innerText_f576815d138c00a8: function(e, t, n) {
                    e.innerText = f(t, n);
                },
                __wbg_set_width_d2ec5d6689655fa9: function(e, t) {
                    e.width = t >>> 0;
                },
                __wbg_shiftKey_2380f1b5c0ab0a0d: function(e) {
                    return e.shiftKey;
                },
                __wbg_shiftKey_8896b6760df23dca: function(e) {
                    return e.shiftKey;
                },
                __wbg_stack_3b0d974bbf31e44f: function(e, t) {
                    const n = t.stack, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    s().setInt32(e + 4, c, !0), s().setInt32(e + 0, _, !0);
                },
                __wbg_static_accessor_GLOBAL_THIS_1c7f1bd6c6941fdb: function() {
                    const e = typeof globalThis > "u" ? null : globalThis;
                    return a(e) ? 0 : d(e);
                },
                __wbg_static_accessor_GLOBAL_e039bc914f83e74e: function() {
                    const e = typeof global > "u" ? null : global;
                    return a(e) ? 0 : d(e);
                },
                __wbg_static_accessor_SELF_8bf8c48c28420ad5: function() {
                    const e = typeof self > "u" ? null : self;
                    return a(e) ? 0 : d(e);
                },
                __wbg_static_accessor_WINDOW_6aeee9b51652ee0f: function() {
                    const e = typeof window > "u" ? null : window;
                    return a(e) ? 0 : d(e);
                },
                __wbg_style_ad734f3851a343fb: function(e) {
                    return e.style;
                },
                __wbg_then_20a157d939b514f5: function(e, t) {
                    return e.then(t);
                },
                __wbg_then_5ef9b762bc91555c: function(e, t, n) {
                    return e.then(t, n);
                },
                __wbg_top_48ee6b46ac920115: function(e) {
                    return e.top;
                },
                __wbg_touches_6674d21130efd19f: function(e) {
                    return e.touches;
                },
                __wbg_width_73079be53f70e8ba: function(e) {
                    return e.width;
                },
                __wbg_width_745cdbb52ce771fd: function(e) {
                    return e.width;
                },
                __wbg_writeText_8da2a080a8f02fcd: function(e, t, n) {
                    return e.writeText(f(t, n));
                },
                __wbg_write_72848bb311dff335: function() {
                    return u(function(e, t, n) {
                        return e.write(f(t, n));
                    }, arguments);
                },
                __wbindgen_cast_0000000000000001: function(e, t) {
                    return g(e, t, P);
                },
                __wbindgen_cast_0000000000000002: function(e, t) {
                    return g(e, t, M);
                },
                __wbindgen_cast_0000000000000003: function(e, t) {
                    return H(e, t, O);
                },
                __wbindgen_cast_0000000000000004: function(e, t) {
                    return g(e, t, D);
                },
                __wbindgen_cast_0000000000000005: function(e, t) {
                    return g(e, t, W);
                },
                __wbindgen_cast_0000000000000006: function(e, t) {
                    return g(e, t, K);
                },
                __wbindgen_cast_0000000000000007: function(e, t) {
                    return g(e, t, F);
                },
                __wbindgen_cast_0000000000000008: function(e, t) {
                    return g(e, t, U);
                },
                __wbindgen_cast_0000000000000009: function(e, t) {
                    return g(e, t, B);
                },
                __wbindgen_cast_000000000000000a: function(e, t) {
                    return g(e, t, L);
                },
                __wbindgen_cast_000000000000000b: function(e, t) {
                    return f(e, t);
                },
                __wbindgen_init_externref_table: function() {
                    const e = o.__wbindgen_externrefs, t = e.grow(4);
                    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, !0), e.set(t + 3, !1);
                }
            }
        };
    }
    function L(r, e) {
        o.wasm_bindgen__convert__closures_____invoke__h7c2623bdbb4ce0cc(r, e);
    }
    function M(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768(r, e, t);
    }
    function O(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__hed8726e231ea9a5a(r, e, t);
    }
    function D(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_3(r, e, t);
    }
    function W(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_4(r, e, t);
    }
    function K(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_5(r, e, t);
    }
    function F(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_6(r, e, t);
    }
    function U(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_7(r, e, t);
    }
    function B(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4dbee921d92b3768_8(r, e, t);
    }
    function P(r, e, t) {
        const n = o.wasm_bindgen__convert__closures_____invoke__hdd56e81b2da19b35(r, e, t);
        if (n[1]) throw V(n[0]);
    }
    function d(r) {
        const e = o.__externref_table_alloc();
        return o.__wbindgen_externrefs.set(e, r), e;
    }
    const k = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((r)=>o.__wbindgen_destroy_closure(r.a, r.b));
    function x(r) {
        const e = typeof r;
        if (e == "number" || e == "boolean" || r == null) return `${r}`;
        if (e == "string") return `"${r}"`;
        if (e == "symbol") {
            const _ = r.description;
            return _ == null ? "Symbol" : `Symbol(${_})`;
        }
        if (e == "function") {
            const _ = r.name;
            return typeof _ == "string" && _.length > 0 ? `Function(${_})` : "Function";
        }
        if (Array.isArray(r)) {
            const _ = r.length;
            let c = "[";
            _ > 0 && (c += x(r[0]));
            for(let i = 1; i < _; i++)c += ", " + x(r[i]);
            return c += "]", c;
        }
        const t = /\[object ([^\]]+)\]/.exec(toString.call(r));
        let n;
        if (t && t.length > 1) n = t[1];
        else return toString.call(r);
        if (n == "Object") try {
            return "Object(" + JSON.stringify(r) + ")";
        } catch  {
            return "Object";
        }
        return r instanceof Error ? `${r.name}: ${r.message}
${r.stack}` : n;
    }
    function j(r, e) {
        return r = r >>> 0, v().subarray(r / 1, r / 1 + e);
    }
    function q(r, e) {
        return r = r >>> 0, N().subarray(r / 1, r / 1 + e);
    }
    let y = null;
    function s() {
        return (y === null || y.buffer.detached === !0 || y.buffer.detached === void 0 && y.buffer !== o.memory.buffer) && (y = new DataView(o.memory.buffer)), y;
    }
    function f(r, e) {
        return Y(r >>> 0, e);
    }
    let h = null;
    function v() {
        return (h === null || h.byteLength === 0) && (h = new Uint8Array(o.memory.buffer)), h;
    }
    let p = null;
    function N() {
        return (p === null || p.byteLength === 0) && (p = new Uint8ClampedArray(o.memory.buffer)), p;
    }
    function u(r, e) {
        try {
            return r.apply(this, e);
        } catch (t) {
            const n = d(t);
            o.__wbindgen_exn_store(n);
        }
    }
    function a(r) {
        return r == null;
    }
    function H(r, e, t) {
        const n = {
            a: r,
            b: e,
            cnt: 1
        }, _ = (...c)=>{
            n.cnt++;
            try {
                return t(n.a, n.b, ...c);
            } finally{
                _._wbg_cb_unref();
            }
        };
        return _._wbg_cb_unref = ()=>{
            --n.cnt === 0 && (o.__wbindgen_destroy_closure(n.a, n.b), n.a = 0, k.unregister(n));
        }, k.register(_, n, n), _;
    }
    function g(r, e, t) {
        const n = {
            a: r,
            b: e,
            cnt: 1
        }, _ = (...c)=>{
            n.cnt++;
            const i = n.a;
            n.a = 0;
            try {
                return t(i, n.b, ...c);
            } finally{
                n.a = i, _._wbg_cb_unref();
            }
        };
        return _._wbg_cb_unref = ()=>{
            --n.cnt === 0 && (o.__wbindgen_destroy_closure(n.a, n.b), n.a = 0, k.unregister(n));
        }, k.register(_, n, n), _;
    }
    function w(r, e, t) {
        if (t === void 0) {
            const b = I.encode(r), m = e(b.length, 1) >>> 0;
            return v().subarray(m, m + b.length).set(b), l = b.length, m;
        }
        let n = r.length, _ = e(n, 1) >>> 0;
        const c = v();
        let i = 0;
        for(; i < n; i++){
            const b = r.charCodeAt(i);
            if (b > 127) break;
            c[_ + i] = b;
        }
        if (i !== n) {
            i !== 0 && (r = r.slice(i)), _ = t(_, n, n = i + r.length * 3, 1) >>> 0;
            const b = v().subarray(_ + i, _ + n), m = I.encodeInto(r, b);
            i += m.written, _ = t(_, n, i, 1) >>> 0;
        }
        return l = i, _;
    }
    function V(r) {
        const e = o.__wbindgen_externrefs.get(r);
        return o.__externref_table_dealloc(r), e;
    }
    let T = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    T.decode();
    const X = 2146435072;
    let E = 0;
    function Y(r, e) {
        return E += e, E >= X && (T = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), T.decode(), E = e), T.decode(v().subarray(r, r + e));
    }
    const I = new TextEncoder;
    "encodeInto" in I || (I.encodeInto = function(r, e) {
        const t = I.encode(r);
        return e.set(t), {
            read: r.length,
            written: t.length
        };
    });
    let l = 0, o;
    function $(r, e) {
        return o = r.exports, y = null, h = null, p = null, o.__wbindgen_start(), o;
    }
    async function z(r, e) {
        if (typeof Response == "function" && r instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming == "function") try {
                return await WebAssembly.instantiateStreaming(r, e);
            } catch (_) {
                if (r.ok && t(r.type) && r.headers.get("Content-Type") !== "application/wasm") console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", _);
                else throw _;
            }
            const n = await r.arrayBuffer();
            return await WebAssembly.instantiate(n, e);
        } else {
            const n = await WebAssembly.instantiate(r, e);
            return n instanceof WebAssembly.Instance ? {
                instance: n,
                module: r
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
    async function G(r) {
        if (o !== void 0) return o;
        r !== void 0 && (Object.getPrototypeOf(r) === Object.prototype ? { module_or_path: r } = r : console.warn("using deprecated parameters for the initialization function; pass a single object instead")), r === void 0 && (r = new URL("" + new URL("rusticon_bg-ZztEB3VT.wasm", import.meta.url).href, import.meta.url));
        const e = S();
        (typeof r == "string" || typeof Request == "function" && r instanceof Request || typeof URL == "function" && r instanceof URL) && (r = fetch(r));
        const { instance: t, module: n } = await z(await r, e);
        return $(t);
    }
    await G();
    C();
})();
