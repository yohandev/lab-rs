import m from 'mithril'
import mod from './lib.rs'

(async () =>
{
    const { add } = await mod()

    m.render(document.body, <div>Hello, World! 3 + 4 = {add(3, 4)}</div>)
})()