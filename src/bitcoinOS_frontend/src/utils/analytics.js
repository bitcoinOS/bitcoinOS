import ReactGA from 'react-ga'

export function initializeAnalytics() {
    ReactGA.initialize('G-CK6DVBSGXN')
}

export function trackPageView(page) {
    ReactGA.event({
        category: 'User',
        action: 'Tested GA Integration'
    });
    ReactGA.pageview(page)
}