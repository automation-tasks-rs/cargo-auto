
'use strict';

// Incrementing VERSION in CACHE_NAME will kick off the 
// install event and force previously cached
// resources to be cached again.
// but the new service worker will not be activated until all 
// tabs with this webapp are closed.

const CACHE_NAME = '2024.419.1740';

self.addEventListener('install', event => {
    console.log('event install ', CACHE_NAME);
    // the ugly trick of avoiding the waiting phase
    self.skipWaiting();

    event.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll(
                [
                    '/pwa_short_name/',
                    'index.html',
                    'favicon.ico',
                    'manifest.json',
                    'start_service_worker.js',
                    'css/basic_style.css',
                    'css/fa-solid-900.woff2',
                    'css/fontawesome.css',
                    'css/normalize.css',
                    'css/Roboto-Medium.woff2',
                    'icons/icon-032.png',
                    'icons/icon-072.png',
                    'icons/icon-096.png',
                    'icons/icon-120.png',
                    'icons/icon-128.png',
                    'icons/icon-144.png',
                    'icons/icon-152.png',
                    'icons/icon-167.png',
                    'icons/icon-180.png',
                    'icons/icon-192.png',
                    'icons/icon-196.png',
                    'icons/icon-512.png',
                    'icons/icon-maskable.png',
                    'pkg/rust_project_name_bg.wasm',
                    'pkg/rust_project_name.js'
                ]
            );
        })
    );
});

self.addEventListener('activate', event => {
    console.log('event activate');
    // Delete all caches that aren't CACHE_NAME.
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (CACHE_NAME.indexOf(cacheName) === -1) {
                        // If this cache name isn't right, then delete it.
                        console.log('Deleting out of date cache:', cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener('fetch', event => {
    // console.log('event fetch');
    // Let the browser do its default thing
    // for non-GET requests.
    if (event.request.method != 'GET') return;

    // Prevent the default, and handle the request ourselves.
    event.respondWith(async function () {
        // Try to get the response from a cache.
        const cache = await caches.open(CACHE_NAME);
        const cachedResponse = await cache.match(event.request);

        if (cachedResponse) {
            // console.log('from cache');
            // If we found a match in the cache, return it, but also
            // update the entry in the cache in the background.
            event.waitUntil(cache.add(event.request));
            return cachedResponse;
        }

        // If we didn't find a match in the cache, use the network and cache it for later.
        const response = await fetch(event.request);
        cache.put(event.request, response.clone());
        return response;
    }());
});
