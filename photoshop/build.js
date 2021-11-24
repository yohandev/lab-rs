const path = require('path')
const fs = require('fs')

const exec = require('util').promisify(require('child_process').exec)

const rs_plugin =
{
    name: 'rust',
    setup: build =>
    {
        build.onResolve({ filter: /\.rs$/ }, args =>
        {
            if (args.namespace === 'wasm-stub')
            {
                return { path: args.path, namespace: 'wasm-binary' }
            }
            if (args.resolveDir === '') { return }
            
            const p = path.isAbsolute(args.path) ? args.path : path.join(args.resolveDir, args.path)
            return { path: p, namespace: 'wasm-stub' }
        })

        build.onLoad({ filter: /.*/, namespace: 'wasm-stub' }, async args =>
        ({
            contents: `
                import bytes from ${JSON.stringify(args.path)}
            
                export default imports => WebAssembly
                    .instantiate(bytes, imports)
                    .then(res => res.instance.exports)
            `
        }))

        build.onLoad({ filter: /.*/, namespace: 'wasm-binary' }, async args =>
        {
            const cwd = path.dirname(args.path)
            
            // Locate project
            const cargo = (await exec("cargo locate-project --message-format plain", { cwd })).stdout
            const wasm = path.join(
                path.dirname(cargo),
                "target",
                "wasm32-unknown-unknown",
                "debug",
                "photoshop.wasm"
            )

            // Build rust project
            const cmd = "cargo"
            const opt = ["build", "--target", "wasm32-unknown-unknown"]

            // Build
            await exec(`${cmd} ${opt.join(' ')}`, { cwd })
            // Minify
            await exec(`wasm-gc ${wasm}`)

            // Output `.wasm` file
            const bin = await fs.promises.readFile(wasm)

            return { contents: bin, loader: 'binary' }
        })
    }
}
const config =
{
    entryPoints: ['src/index.js'],
    
    plugins: [rs_plugin],
    inject: ['./mithril-shim.js'],

    target: 'es6',
    loader: { '.js': 'jsx' },
    jsxFactory: 'm',
    jsxFragment: 'm.Fragment',
    
    bundle: true,
    minify: true,

    outdir: 'pkg'
}

if (process.argv.includes('--serve'))
{
    require('esbuild')
        .serve({ servedir: 'pkg' }, config)
        .then(server => console.log(`Serving on http://127.0.0.1:${server.port}`))
}
else
{
    require('esbuild')
        .build(config)
        .catch(_ => process.exit(1))
}