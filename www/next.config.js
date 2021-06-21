module.exports = {
  webpack: (config, { buildId, dev, isServer, defaultLoaders, webpack }) => {
    // Important: return the modified config
    config.experiments = {
      syncWebAssembly: true,
    };
    return config;
  },
};
