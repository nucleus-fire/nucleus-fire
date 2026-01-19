<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SaaS Starter</title>
    <link href="/assets/styles.css" rel="stylesheet">
</head>
<body class="bg-gray-50 min-h-screen flex flex-col font-sans text-gray-900">

    <!-- Top Navigation -->
    <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16">
                <div class="flex">
                    <div class="flex-shrink-0 flex items-center">
                        <span class="font-bold text-xl text-primary-600">Rocket<span class="text-gray-900">SaaS</span></span>
                    </div>
                    <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                        <a href="/" class="border-primary-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                            Home
                        </a>
                        <a href="/pricing" class="border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                            Pricing
                        </a>
                    </div>
                </div>
                <div class="hidden sm:ml-6 sm:flex sm:items-center space-x-4">
                    <a href="/login" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">Log in</a>
                    <a href="/register" class="bg-primary-600 hover:bg-primary-700 text-white px-4 py-2 rounded-md text-sm font-medium transition shadow-sm">Get Started</a>
                </div>
            </div>
        </div>
    </nav>
    
    <!-- Main Content -->
    <main class="flex-1">
        <slot />
    </main>
    
    <!-- Footer -->
    <footer class="bg-white border-t border-gray-200 mt-auto">
        <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row justify-between items-center">
             <div class="flex space-x-6 order-2 md:order-1 mt-4 md:mt-0">
                <p class="text-gray-400 text-sm">&copy; 2026 RocketSaaS Inc. All rights reserved.</p>
             </div>
             <div class="flex space-x-6 order-1 md:order-2">
                 <a href="#" class="text-gray-400 hover:text-gray-500 text-sm">Privacy</a>
                 <a href="#" class="text-gray-400 hover:text-gray-500 text-sm">Terms</a>
             </div>
        </div>
    </footer>

</body>
</html>
