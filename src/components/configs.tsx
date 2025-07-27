import { useEffect, useState } from "react";
import { configManager, PeeksyConfig } from "../lib/configManager";
import ConfigsItem from "./configsItem";

const Configs = () => {
  const [config, setConfig] = useState<PeeksyConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      setLoading(true);
      setError(null);
      const fetchedConfig = await configManager.getConfig();
      setConfig(fetchedConfig);
    } catch (err: any) {
      setError(err.message || "Failed to load configuration");
      console.error("Failed to load config:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleFieldUpdate = async (
    field: keyof PeeksyConfig,
    value: string
  ) => {
    try {
      setError(null);
      const updatedConfig = await configManager.updateField(field, value);
      setConfig(updatedConfig);
      return true;
    } catch (err: any) {
      setError(err.message || `Failed to update ${field}`);
      console.error(`Failed to update ${field}:`, err);
      return false;
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="text-gray-600 dark:text-gray-400">
          Loading configuration...
        </div>
      </div>
    );
  }

  if (error && !config) {
    return (
      <div className="flex flex-col items-center gap-4 p-8">
        <div className="text-red-600 dark:text-red-400">Error: {error}</div>
        <button
          onClick={loadConfig}
          className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div>
      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 border border-red-300 dark:border-red-700 rounded-md">
          <div className="text-red-800 dark:text-red-200 text-sm">{error}</div>
        </div>
      )}

      <div className="flex flex-col gap-4">
        <ConfigsItem
          label="OpenAI API Key"
          value={config?.openai_api_key ?? ""}
          placeholder="Enter your OpenAI API key (sk-...)"
          onSave={(value) => handleFieldUpdate("openai_api_key", value)}
          sensitive={true}
        />

        {/* <ConfigsItem
          label="Peeksy Prompt"
          value={config?.peeksy_prompt ?? ""}
          placeholder="Enter your custom prompt for image analysis"
          onSave={(value) => handleFieldUpdate("peeksy_prompt", value)}
          multiline={true}
        />

        <ConfigsItem
          label="OpenAI Model"
          value={config?.openai_model ?? ""}
          placeholder="e.g. gpt-4o, gpt-4o-mini, gpt-4-turbo"
          onSave={(value) => handleFieldUpdate("openai_model", value)}
        /> */}
      </div>
    </div>
  );
};

export default Configs;
