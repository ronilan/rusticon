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
                __wbg___wbindgen_debug_string_0accd80f45e5faa2: function(e, t) {
                    const n = A(t), _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg___wbindgen_is_function_754e9f305ff6029e: function(e) {
                    return typeof e == "function";
                },
                __wbg___wbindgen_is_undefined_67b456be8673d3d7: function(e) {
                    return e === void 0;
                },
                __wbg___wbindgen_jsval_eq_1068e624fa87f6ab: function(e, t) {
                    return e === t;
                },
                __wbg___wbindgen_number_get_9bb1761122181af2: function(e, t) {
                    const n = t, _ = typeof n == "number" ? n : void 0;
                    a().setFloat64(e + 8, f(_) ? 0 : _, !0), a().setInt32(e + 0, !f(_), !0);
                },
                __wbg___wbindgen_string_get_72bdf95d3ae505b1: function(e, t) {
                    const n = t, _ = typeof n == "string" ? n : void 0;
                    var c = f(_) ? 0 : w(_, o.__wbindgen_malloc, o.__wbindgen_realloc), i = l;
                    a().setInt32(e + 4, i, !0), a().setInt32(e + 0, c, !0);
                },
                __wbg___wbindgen_throw_1506f2235d1bdba0: function(e, t) {
                    throw new Error(u(e, t));
                },
                __wbg__wbg_cb_unref_61db23ac97f16c31: function(e) {
                    e._wbg_cb_unref();
                },
                __wbg_addEventListener_7c5a0db2b2826a06: function() {
                    return s(function(e, t, n, _) {
                        e.addEventListener(u(t, n), _);
                    }, arguments);
                },
                __wbg_altKey_4efe9bf67d712839: function(e) {
                    return e.altKey;
                },
                __wbg_altKey_77d5df8df54f3c7e: function(e) {
                    return e.altKey;
                },
                __wbg_appendChild_364435158a70bd03: function() {
                    return s(function(e, t) {
                        return e.appendChild(t);
                    }, arguments);
                },
                __wbg_arrayBuffer_a0e88fd0c0e099b2: function(e) {
                    return e.arrayBuffer();
                },
                __wbg_body_7d5b1a2ac7f2c821: function(e) {
                    const t = e.body;
                    return f(t) ? 0 : d(t);
                },
                __wbg_button_f3dc4c82e6ee9a0c: function(e) {
                    return e.button;
                },
                __wbg_buttons_8dae14f7d9ea8c8a: function(e) {
                    return e.buttons;
                },
                __wbg_call_8a89609d89f6608a: function() {
                    return s(function(e, t) {
                        return e.call(t);
                    }, arguments);
                },
                __wbg_changedTouches_f4bc81aa9275b3b1: function(e) {
                    return e.changedTouches;
                },
                __wbg_clearRect_c6c4586d143d768c: function(e, t, n, _, c) {
                    e.clearRect(t, n, _, c);
                },
                __wbg_clientX_6a613cbcceb44e9d: function(e) {
                    return e.clientX;
                },
                __wbg_clientX_c85019015e605e82: function(e) {
                    return e.clientX;
                },
                __wbg_clientY_48b9212040ca1133: function(e) {
                    return e.clientY;
                },
                __wbg_clientY_e89b1cdbdb6c1772: function(e) {
                    return e.clientY;
                },
                __wbg_clipboardData_9cee642a6a4aa59c: function(e) {
                    const t = e.clipboardData;
                    return f(t) ? 0 : d(t);
                },
                __wbg_clipboard_53ba93c4ad802c3d: function(e) {
                    return e.clipboard;
                },
                __wbg_close_3a6bb6c95fcc0a4a: function(e) {
                    return e.close();
                },
                __wbg_createElement_c3c16a9aa7f5cc74: function() {
                    return s(function(e, t, n) {
                        return e.createElement(u(t, n));
                    }, arguments);
                },
                __wbg_createWritable_d19f7c689e379eac: function(e) {
                    return e.createWritable();
                },
                __wbg_ctrlKey_1cae6780759a470d: function(e) {
                    return e.ctrlKey;
                },
                __wbg_ctrlKey_a1ca4695e4fe525a: function(e) {
                    return e.ctrlKey;
                },
                __wbg_dataTransfer_bf723904983b1674: function(e) {
                    const t = e.dataTransfer;
                    return f(t) ? 0 : d(t);
                },
                __wbg_deltaX_5193430f52fdad3f: function(e) {
                    return e.deltaX;
                },
                __wbg_deltaY_e1fefa22823d87c4: function(e) {
                    return e.deltaY;
                },
                __wbg_detail_8d603e8cc69062d8: function(e) {
                    return e.detail;
                },
                __wbg_devicePixelRatio_dab1a0b7ea57b26a: function(e) {
                    return e.devicePixelRatio;
                },
                __wbg_document_aceb08cd6489baf5: function(e) {
                    const t = e.document;
                    return f(t) ? 0 : d(t);
                },
                __wbg_drawImage_5672faddb3fa4b97: function() {
                    return s(function(e, t, n, _, c, i, b, m, E, R) {
                        e.drawImage(t, n, _, c, i, b, m, E, R);
                    }, arguments);
                },
                __wbg_error_a6fa202b58aa1cd3: function(e, t) {
                    let n, _;
                    try {
                        n = e, _ = t, console.error(u(e, t));
                    } finally{
                        o.__wbindgen_free(n, _, 1);
                    }
                },
                __wbg_fillRect_3916c35e6834cd1b: function(e, t, n, _, c) {
                    e.fillRect(t, n, _, c);
                },
                __wbg_getBoundingClientRect_93c2750834277567: function(e) {
                    return e.getBoundingClientRect();
                },
                __wbg_getComputedStyle_c59f58a15bc6a800: function() {
                    return s(function(e, t) {
                        const n = e.getComputedStyle(t);
                        return f(n) ? 0 : d(n);
                    }, arguments);
                },
                __wbg_getContext_469d34698d869fc1: function() {
                    return s(function(e, t, n) {
                        const _ = e.getContext(u(t, n));
                        return f(_) ? 0 : d(_);
                    }, arguments);
                },
                __wbg_getData_ac647d2fd3233198: function() {
                    return s(function(e, t, n, _) {
                        const c = t.getData(u(n, _)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        a().setInt32(e + 4, b, !0), a().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_getElementById_c35b4b7d270d161d: function(e, t, n) {
                    const _ = e.getElementById(u(t, n));
                    return f(_) ? 0 : d(_);
                },
                __wbg_getFile_57f23539bd8b1536: function(e) {
                    return e.getFile();
                },
                __wbg_getPropertyValue_dbbb77f232017e4d: function() {
                    return s(function(e, t, n, _) {
                        const c = t.getPropertyValue(u(n, _)), i = w(c, o.__wbindgen_malloc, o.__wbindgen_realloc), b = l;
                        a().setInt32(e + 4, b, !0), a().setInt32(e + 0, i, !0);
                    }, arguments);
                },
                __wbg_get_c4eb9c8665471988: function(e, t) {
                    const n = e[t >>> 0];
                    return f(n) ? 0 : d(n);
                },
                __wbg_get_dd98bf4577cc33b4: function(e, t) {
                    const n = e[t >>> 0];
                    return f(n) ? 0 : d(n);
                },
                __wbg_get_de6a0f7d4d18a304: function() {
                    return s(function(e, t) {
                        return Reflect.get(e, t);
                    }, arguments);
                },
                __wbg_height_8e3b6ac1a60655fb: function(e) {
                    return e.height;
                },
                __wbg_height_ef5b5950872773b5: function(e) {
                    return e.height;
                },
                __wbg_innerHeight_8b6ee2571dbedb9d: function() {
                    return s(function(e) {
                        return e.innerHeight;
                    }, arguments);
                },
                __wbg_innerWidth_7475bec19f48fe43: function() {
                    return s(function(e) {
                        return e.innerWidth;
                    }, arguments);
                },
                __wbg_instanceof_CanvasRenderingContext2d_6f2951bc60fb6d97: function(e) {
                    let t;
                    try {
                        t = e instanceof CanvasRenderingContext2D;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlCanvasElement_8325b7578cc1684c: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLCanvasElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_HtmlElement_9d326f7a42217802: function(e) {
                    let t;
                    try {
                        t = e instanceof HTMLElement;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_instanceof_Window_e093be59ee9a8e14: function(e) {
                    let t;
                    try {
                        t = e instanceof Window;
                    } catch  {
                        t = !1;
                    }
                    return t;
                },
                __wbg_items_2c4524d8f08df46b: function(e) {
                    return e.items;
                },
                __wbg_key_df6a54e3e036c3fe: function(e, t) {
                    const n = t.key, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg_kind_9d0d4edd90c193cf: function(e, t) {
                    const n = t.kind, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg_left_985fa07b897d8e74: function(e) {
                    return e.left;
                },
                __wbg_length_4a591ecaa01354d9: function(e) {
                    return e.length;
                },
                __wbg_length_f1b1258ecc19cc7d: function(e) {
                    return e.length;
                },
                __wbg_metaKey_752862905c708ca9: function(e) {
                    return e.metaKey;
                },
                __wbg_metaKey_d2a47aa621ff2c45: function(e) {
                    return e.metaKey;
                },
                __wbg_name_2b1afe861fc08636: function(e, t) {
                    const n = t.name, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg_name_e1829143ad527577: function(e, t) {
                    const n = t.name, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg_navigator_3833ecdbc19d2757: function(e) {
                    return e.navigator;
                },
                __wbg_new_227d7c05414eb861: function() {
                    return new Error;
                },
                __wbg_new_578aeef4b6b94378: function(e) {
                    return new Uint8Array(e);
                },
                __wbg_new_with_u8_clamped_array_and_sh_e3609225f4ad3a74: function() {
                    return s(function(e, t, n, _) {
                        return new ImageData(q(e, t), n >>> 0, _ >>> 0);
                    }, arguments);
                },
                __wbg_now_190933fa139cc119: function() {
                    return Date.now();
                },
                __wbg_parentNode_6ee3190f9ba96b9b: function(e) {
                    const t = e.parentNode;
                    return f(t) ? 0 : d(t);
                },
                __wbg_preventDefault_4902f41a1b31bedd: function(e) {
                    e.preventDefault();
                },
                __wbg_prototypesetcall_3249fc62a0fafa30: function(e, t, n) {
                    Uint8Array.prototype.set.call(j(e, t), n);
                },
                __wbg_putImageData_9118a61bc5ed588d: function() {
                    return s(function(e, t, n, _) {
                        e.putImageData(t, n, _);
                    }, arguments);
                },
                __wbg_querySelector_6f6509bf1f8f4753: function() {
                    return s(function(e, t, n) {
                        const _ = e.querySelector(u(t, n));
                        return f(_) ? 0 : d(_);
                    }, arguments);
                },
                __wbg_queueMicrotask_35c611f4a14830b2: function(e) {
                    queueMicrotask(e);
                },
                __wbg_queueMicrotask_404ed0a58e0b63cc: function(e) {
                    return e.queueMicrotask;
                },
                __wbg_removeChild_542662c726ba0cbf: function() {
                    return s(function(e, t) {
                        return e.removeChild(t);
                    }, arguments);
                },
                __wbg_resolve_25a7e548d5881dca: function(e) {
                    return Promise.resolve(e);
                },
                __wbg_setAttribute_5b695d1c3be2e3e6: function() {
                    return s(function(e, t, n, _, c) {
                        e.setAttribute(u(t, n), u(_, c));
                    }, arguments);
                },
                __wbg_setProperty_a6e0b14612e307b1: function() {
                    return s(function(e, t, n, _, c) {
                        e.setProperty(u(t, n), u(_, c));
                    }, arguments);
                },
                __wbg_setTimeout_b5f25e402b6e8ff9: function() {
                    return s(function(e, t, n) {
                        return e.setTimeout(t, n);
                    }, arguments);
                },
                __wbg_setTransform_a492a4c5445a583a: function() {
                    return s(function(e, t, n, _, c, i, b) {
                        e.setTransform(t, n, _, c, i, b);
                    }, arguments);
                },
                __wbg_set_fillStyle_e96ce0e2e6d0bd13: function(e, t, n) {
                    e.fillStyle = u(t, n);
                },
                __wbg_set_height_0739170de8653cc4: function(e, t) {
                    e.height = t >>> 0;
                },
                __wbg_set_id_b9d2ee0b28d87959: function(e, t, n) {
                    e.id = u(t, n);
                },
                __wbg_set_imageSmoothingEnabled_f35b1f163ed4a14d: function(e, t) {
                    e.imageSmoothingEnabled = t !== 0;
                },
                __wbg_set_innerHTML_6bcbbce0a3626998: function(e, t, n) {
                    e.innerHTML = u(t, n);
                },
                __wbg_set_innerText_2126c17ae88dc653: function(e, t, n) {
                    e.innerText = u(t, n);
                },
                __wbg_set_title_bd6edc450bc7efb8: function(e, t, n) {
                    e.title = u(t, n);
                },
                __wbg_set_width_87301412247f3343: function(e, t) {
                    e.width = t >>> 0;
                },
                __wbg_shiftKey_05941b44ffe0a9ce: function(e) {
                    return e.shiftKey;
                },
                __wbg_shiftKey_ec95aec36c86fb31: function(e) {
                    return e.shiftKey;
                },
                __wbg_stack_3b0d974bbf31e44f: function(e, t) {
                    const n = t.stack, _ = w(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = l;
                    a().setInt32(e + 4, c, !0), a().setInt32(e + 0, _, !0);
                },
                __wbg_static_accessor_GLOBAL_9d53f2689e622ca1: function() {
                    const e = typeof global > "u" ? null : global;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_GLOBAL_THIS_a1a35cec07001a8a: function() {
                    const e = typeof globalThis > "u" ? null : globalThis;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_SELF_4c59f6c7ea29a144: function() {
                    const e = typeof self > "u" ? null : self;
                    return f(e) ? 0 : d(e);
                },
                __wbg_static_accessor_WINDOW_e70ae9f2eb052253: function() {
                    const e = typeof window > "u" ? null : window;
                    return f(e) ? 0 : d(e);
                },
                __wbg_style_ad0f3eb1fd1aa2bc: function(e) {
                    return e.style;
                },
                __wbg_then_18f476d590e58992: function(e, t, n) {
                    return e.then(t, n);
                },
                __wbg_then_ac7b025999b52837: function(e, t) {
                    return e.then(t);
                },
                __wbg_top_14d766e5bde56568: function(e) {
                    return e.top;
                },
                __wbg_touches_65e436ac07e6aa05: function(e) {
                    return e.touches;
                },
                __wbg_width_1e0b74fef17bc28b: function(e) {
                    return e.width;
                },
                __wbg_width_796e38875beab5e6: function(e) {
                    return e.width;
                },
                __wbg_writeText_e35fb5c1924eb2ac: function(e, t, n) {
                    return e.writeText(u(t, n));
                },
                __wbg_write_6d8582785bb84650: function() {
                    return s(function(e, t, n) {
                        return e.write(u(t, n));
                    }, arguments);
                },
                __wbindgen_cast_0000000000000001: function(e, t) {
                    return g(e, t, P);
                },
                __wbindgen_cast_0000000000000002: function(e, t) {
                    return g(e, t, O);
                },
                __wbindgen_cast_0000000000000003: function(e, t) {
                    return H(e, t, D);
                },
                __wbindgen_cast_0000000000000004: function(e, t) {
                    return g(e, t, M);
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
                    return u(e, t);
                },
                __wbindgen_init_externref_table: function() {
                    const e = o.__wbindgen_externrefs, t = e.grow(4);
                    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, !0), e.set(t + 3, !1);
                }
            }
        };
    }
    function L(r, e) {
        o.wasm_bindgen__convert__closures_____invoke__h76eb8253719278fb(r, e);
    }
    function O(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c(r, e, t);
    }
    function D(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h4f1b80ea0322dd0b(r, e, t);
    }
    function M(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_3(r, e, t);
    }
    function W(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_4(r, e, t);
    }
    function K(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_5(r, e, t);
    }
    function F(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_6(r, e, t);
    }
    function U(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_7(r, e, t);
    }
    function B(r, e, t) {
        o.wasm_bindgen__convert__closures_____invoke__h24b13a9ac7125b1c_8(r, e, t);
    }
    function P(r, e, t) {
        const n = o.wasm_bindgen__convert__closures_____invoke__h16416f7c1cc3f264(r, e, t);
        if (n[1]) throw X(n[0]);
    }
    function d(r) {
        const e = o.__externref_table_alloc();
        return o.__wbindgen_externrefs.set(e, r), e;
    }
    const k = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((r)=>o.__wbindgen_destroy_closure(r.a, r.b));
    function A(r) {
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
            _ > 0 && (c += A(r[0]));
            for(let i = 1; i < _; i++)c += ", " + A(r[i]);
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
    function a() {
        return (y === null || y.buffer.detached === !0 || y.buffer.detached === void 0 && y.buffer !== o.memory.buffer) && (y = new DataView(o.memory.buffer)), y;
    }
    function u(r, e) {
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
    function s(r, e) {
        try {
            return r.apply(this, e);
        } catch (t) {
            const n = d(t);
            o.__wbindgen_exn_store(n);
        }
    }
    function f(r) {
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
    function X(r) {
        const e = o.__wbindgen_externrefs.get(r);
        return o.__externref_table_dealloc(r), e;
    }
    let T = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    T.decode();
    const V = 2146435072;
    let x = 0;
    function Y(r, e) {
        return x += e, x >= V && (T = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), T.decode(), x = e), T.decode(v().subarray(r, r + e));
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
        r !== void 0 && (Object.getPrototypeOf(r) === Object.prototype ? { module_or_path: r } = r : console.warn("using deprecated parameters for the initialization function; pass a single object instead")), r === void 0 && (r = new URL("" + new URL("rusticon_bg-CXxORVDT.wasm", import.meta.url).href, import.meta.url));
        const e = S();
        (typeof r == "string" || typeof Request == "function" && r instanceof Request || typeof URL == "function" && r instanceof URL) && (r = fetch(r));
        const { instance: t, module: n } = await z(await r, e);
        return $(t);
    }
    await G();
    C();
})();
