module.exports = {
  workspaceRoot: false,
  mount: {
    www: '/',
    pkg: {
      url: '/brain_games',
      static: true,
      resolve: false,
    },
  },
  optimize: {
    minify: true,
    target: 'esnext',
  },
  plugins: ['@snowpack/plugin-typescript'],
};
