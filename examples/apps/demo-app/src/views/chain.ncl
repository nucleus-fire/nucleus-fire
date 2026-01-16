<n:view title="Blockchain">
    <h1>Web3 Integration</h1>
    <p>Connect wallets and sign messages seamlessly.</p>

    <div class="card glass" style="max-width: 500px">
        <div id="wallet-ui" style="text-align: center;">
            <img src="https://upload.wikimedia.org/wikipedia/commons/3/36/MetaMask_Fox.svg" width="64" style="margin-bottom: 1rem;">
            <h3 style="margin-bottom: 2rem;">Connect your Wallet</h3>
            <button class="btn" onclick="connectWallet()">Connect MetaMask</button>
        </div>

        <div id="connected-ui" style="display: none; text-align: center;">
            <div style="background: rgba(46, 204, 113, 0.2); color: #2ecc71; padding: 0.5rem; border-radius: 6px; margin-bottom: 1.5rem; display: inline-block;">
                Connected
            </div>
            <p style="font-family: monospace; background: #000; padding: 0.5rem; border-radius: 4px;" id="address-display"></p>
            
            <hr style="border: 0; border-top: 1px solid var(--border); margin: 2rem 0;">
            
            <p>Prove identity by signing a message:</p>
            <button class="btn" onclick="signMessage()">Sign "Login into Nucleus"</button>
        </div>
    </div>

    <script>
        async function connectWallet() {
            // Mock connection
            document.getElementById('wallet-ui').style.display = 'none';
            document.getElementById('connected-ui').style.display = 'block';
            document.getElementById('address-display').innerText = '0x71C7656EC7ab88b098defB751B7401B5f6d8976F';
        }

        async function signMessage() {
            alert("Mock: Message signed! Signature: 0xabc123...");
            // Real logic:
            // window.ethereum.request({ method: 'personal_sign', params: [msg, account] });
        }
    </script>
</n:view>
