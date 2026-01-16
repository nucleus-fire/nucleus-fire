<n:component name="CodeBlock">
<n:props>
    language: String = "rust"
</n:props>

<pre class="code-block"><code class="language-{{ language }}"><n:slot /></code></pre>

<style scoped>
.code-block {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    padding: 20px;
    overflow-x: auto;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 13px;
    line-height: 1.6;
}

.code-block code {
    color: var(--text-primary);
}
</style>
</n:component>
