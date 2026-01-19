<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus Chat</title>
    <title>Nucleus Chat</title>
    <link href="/assets/styles.css" rel="stylesheet">

</head>
<body class="bg-gray-100 dark:bg-gray-900 h-screen flex overflow-hidden font-sans text-gray-900 dark:text-gray-100">

    <!-- Sidebar -->
    <aside class="w-64 bg-gray-900 dark:bg-black border-r border-gray-800 flex flex-col">
        <!-- Team Name -->
        <div class="h-16 flex items-center px-6 border-b border-gray-800 font-bold text-lg tracking-wider">
            DEPLOY<span class="text-blue-500">HQ</span>
        </div>

        <!-- Channels -->
        <div class="flex-1 px-3 py-4 space-y-1 overflow-y-auto">
            <div class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Channels</div>
            <a href="#" class="flex items-center px-3 py-2 text-gray-300 bg-gray-800 rounded-md transition duration-200">
                <span class="text-gray-500 mr-3">#</span> general
            </a>
            <a href="#" class="flex items-center px-3 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition duration-200">
                <span class="text-gray-500 mr-3">#</span> engineering
            </a>
            <a href="#" class="flex items-center px-3 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition duration-200">
                <span class="text-gray-500 mr-3">#</span> marketing
            </a>
            <a href="#" class="flex items-center px-3 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition duration-200">
                <span class="text-gray-500 mr-3">#</span> random
            </a>
            
            <div class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mt-8 mb-2">Direct Messages</div>
            <a href="#" class="flex items-center px-3 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition duration-200">
                <div class="w-2 h-2 rounded-full bg-green-500 mr-3"></div> Sarah
            </a>
            <a href="#" class="flex items-center px-3 py-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition duration-200">
                <div class="w-2 h-2 rounded-full bg-gray-500 mr-3"></div> Mike
            </a>
        </div>
    </aside>

    <!-- Main Chat Area -->
    <main class="flex-1 flex flex-col min-w-0 bg-white dark:bg-gray-900">
        <!-- Header -->
        <header class="h-16 flex items-center justify-between px-6 border-b dark:border-gray-800">
            <div class="flex items-center">
                <div class="text-gray-500 mr-2 text-xl font-light">#</div>
                <h2 class="font-bold text-lg">general</h2>
            </div>
            <div class="flex -space-x-2">
                <img class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-900" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="">
                <img class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-900" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Aneka" alt="">
                <img class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-900" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Bob" alt="">
                <div class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-900 bg-gray-200 flex items-center justify-center text-xs font-bold text-gray-600">+4</div>
            </div>
        </header>

        <!-- Messages -->
        <div id="messages" class="flex-1 overflow-y-auto px-6 py-4 space-y-6 scrollbar-hide">
            <!-- Message 1 -->
            <div class="flex items-start">
                <img class="w-10 h-10 rounded-md mr-4" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="Avatar">
                <div>
                    <div class="flex items-baseline">
                        <span class="font-bold mr-2">Felix</span>
                        <span class="text-xs text-gray-500">10:42 AM</span>
                    </div>
                    <p class="text-gray-800 dark:text-gray-300 mt-1">Has anyone tested the new deployment pipeline?</p>
                </div>
            </div>

            <!-- Message 2 -->
            <div class="flex items-start">
                <img class="w-10 h-10 rounded-md mr-4" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Aneka" alt="Avatar">
                <div>
                    <div class="flex items-baseline">
                        <span class="font-bold mr-2">Aneka</span>
                        <span class="text-xs text-gray-500">10:45 AM</span>
                    </div>
                    <p class="text-gray-800 dark:text-gray-300 mt-1">Yep! <span class="text-blue-500 font-medium">@Felix</span> it's working smooth as butter. 🚀</p>
                </div>
            </div>
        </div>

        <!-- Input Area -->
        <div class="px-6 py-4 pb-6">
            <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-gray-400">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path></svg>
                </div>
                <input type="text" id="chatInput"
                    class="block w-full pl-10 pr-12 py-3 bg-gray-100 dark:bg-gray-800 border-transparent rounded-lg focus:bg-white dark:focus:bg-gray-800 focus:ring-2 focus:ring-blue-500 focus:border-transparent sm:text-sm transition shadow-sm placeholder-gray-500"
                    placeholder="Message #general"
                    autocomplete="off"
                >
                <div class="absolute inset-y-0 right-0 pr-3 flex items-center">
                   <span class="text-xs text-gray-400">Return to send</span>
                </div>
            </div>
        </div>
    </main>

    <script>
        const input = document.getElementById('chatInput');
        const messages = document.getElementById('messages');

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && input.value.trim() !== '') {
                const text = input.value;
                input.value = '';
                
                // Optimistic UI Update
                const msgDiv = document.createElement('div');
                msgDiv.className = 'flex items-start slide-in';
                // Using a random seed for "me"
                msgDiv.innerHTML = `
                    <img class="w-10 h-10 rounded-md mr-4" src="https://api.dicebear.com/7.x/avataaars/svg?seed=Me" alt="Avatar">
                    <div>
                        <div class="flex items-baseline">
                            <span class="font-bold mr-2 text-blue-500">You</span>
                            <span class="text-xs text-gray-500">Just now</span>
                        </div>
                        <p class="text-gray-800 dark:text-gray-300 mt-1">${text}</p>
                    </div>
                `;
                
                messages.appendChild(msgDiv);
                messages.scrollTop = messages.scrollHeight;
            }
        });
    </script>
</body>
</html>
