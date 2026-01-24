/**
 * @template T
 * @typedef {[()=>T,(val:T)=>void]} Signal
 */
/** @type {Array<()=>void>} */
const currentDependentStack = []

/**
 * @template T
 * @param {T} init
 * @returns {Signal<T>}
 */
export function useSignal(init) {
    let value = init
    const dependants = new Set()

    return [
        () => {
            const dep = currentDependentStack[0]
            if (dep) dependants.add(dep)
            return value
        },
        (val) => {
            value = val
            dependants.forEach(fn => fn())
        }
    ]
}

/**
 * @template T
 * @param {()=>T} signal
 */
export function useDerived(signal) {
    /** @type {T} */
    let value;
    useEffect(() => value = signal())
    return () => value
}

/**
 * @param {()=>void} func
 */
export function useEffect(func) {
    currentDependentStack.unshift(func)
    try {
        func()
    } finally {
        currentDependentStack.shift()
    }
}
