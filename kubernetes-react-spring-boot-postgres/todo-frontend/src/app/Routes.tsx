import React from 'react'
import { BrowserRouter, Route } from 'react-router-dom'
import NotFoundPage from '../features/not-found/NotFoundPage'
import App from './App'

const Routes = () => {
    return (
        <BrowserRouter>
            <Route exact path="/">
                <App />
            </Route>
            <Route path="/:path">
                <NotFoundPage />
            </Route>
        </BrowserRouter>
    )
}

export default Routes
