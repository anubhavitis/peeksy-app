import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import ConfigsItem from "./configsItem";

export type Config = {
  openai_api_key: string | null;
  openai_prompt_file_path: string | null;
  openai_model: string | null;
};

const Configs = () => {
  let [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    invoke<Config>("get_config").then((config) => {
      console.log("config received", config);
      setConfig(config);
    });
  }, []);

  return (
    <div>
      <div className="flex flex-col gap-4">
        <ConfigsItem
          label="OpenAI API Key"
          value={config?.openai_api_key ?? ""}
          placeholder="Enter your OpenAI API key"
        />
        <ConfigsItem
          label="OpenAI Prompt File Path"
          value={config?.openai_prompt_file_path ?? ""}
          placeholder="Path to your prompt file"
        />
        <ConfigsItem
          label="OpenAI Model"
          value={config?.openai_model ?? ""}
          placeholder="e.g. gpt-4, gpt-3.5-turbo"
          disabled={true}
        />
      </div>
    </div>
  );
};

export default Configs;
