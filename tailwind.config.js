module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: 'class', // Enables dark mode based on class
    theme: {
      extend: {
        colors: {
            light: {
                100: '#474343'
            },
            primary: {
                900: '#1e1e1e',
                800: '#303030',
                700: '#383838',
                600: '#474343',
                500: '#4e4e4e',
                400: '#616161',
                300: '#aeaeae',
                200: '#bbbbbb',
                100: '#ffffff'
            },
            accent: {
                red: {
                    200: '#C08686',
                    100: '#FFB0B0',
                },
                green: {
                    300: '#5F894D',
                    200: '#74a770',
                    100: '#9ee19a',
                },
                blue: {
                    400: '#0075FF',
                    300: '#012456',
                    200: '#006591',
                    100: '#0178ab',
                }
            }
        },
        boxShadow: {
            'executing': 'inset 0px 0px 20px 0px #12ff77',
        }
      },
    }
}