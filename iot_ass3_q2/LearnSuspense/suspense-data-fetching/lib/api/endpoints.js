import wrapPromise from './wrapPromise'

const pendingURL = 'http://www.mocky.io/v2/5dd7ff583100007400055ced'
const completedURL = 'http://www.mocky.io/v2/5dd7ffde310000b67b055ced'

function fetchPendingTodos() {
    const promise = fetch(pendingURL)
    .then((res) => res.json())
    .then((res) => res.data)

    return wrapPromise(promise)
}

function fetchCompletedTodos() {
    const promise = fetch(completedURL)
    .then((res) => res.json())
    .then((res) => res.data)

    return wrapPromise(promise)
}

export { fetchPendingTodos, fetchCompletedTodos}