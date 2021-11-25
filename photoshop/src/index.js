import m from 'mithril'
import mod from './lib.rs'

(async () =>
{
    const { add, set_panic_hook, memory } = await mod({
        env:
        {
            abort: (ptr, len) =>
            {
                // Reconstitute string
                const buf = new Uint8Array(memory.buffer, ptr, len)
                
                alert(new TextDecoder('UTF-8').decode(buf))
            }
        }
    })

    set_panic_hook()

    m.render(document.body, <div>Hello, World! 3 + 4 = {add(3, 4)}</div>)
})()