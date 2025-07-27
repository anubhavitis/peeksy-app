import { invoke } from "@tauri-apps/api/core";

export interface PeeksyConfig {
  openai_api_key: string;
  peeksy_prompt: string;
  openai_model: string;
  updated_at: number;
}

export class ConfigManager {
  /**
   * Get the current Peeksy configuration
   */
  async getConfig(): Promise<PeeksyConfig> {
    try {
      const config = await invoke<PeeksyConfig>("get_peeksy_config");
      return config;
    } catch (error) {
      throw new Error("Failed to fetch configuration");
    }
  }

  /**
   * Save the complete Peeksy configuration
   */
  async saveConfig(config: PeeksyConfig): Promise<void> {
    try {
      await invoke("save_peeksy_config", { config });
    } catch (error) {
      throw new Error("Failed to save configuration");
    }
  }

  /**
   * Update a specific configuration field
   */
  async updateField(
    field: keyof PeeksyConfig,
    value: string
  ): Promise<PeeksyConfig> {
    try {
      const updatedConfig = await invoke<PeeksyConfig>("update_config_field", {
        field: field.toString(),
        value,
      });
      return updatedConfig;
    } catch (error) {
      throw new Error(`Failed to update ${field}`);
    }
  }

  /**
   * Update OpenAI API Key
   */
  async updateApiKey(apiKey: string): Promise<PeeksyConfig> {
    return this.updateField("openai_api_key", apiKey);
  }

  /**
   * Update Peeksy Prompt
   */
  async updatePrompt(prompt: string): Promise<PeeksyConfig> {
    return this.updateField("peeksy_prompt", prompt);
  }

  /**
   * Update OpenAI Model
   */
  async updateModel(model: string): Promise<PeeksyConfig> {
    return this.updateField("openai_model", model);
  }

  /**
   * Check if configuration is complete and valid
   */
  async isConfigComplete(): Promise<boolean> {
    try {
      const isComplete = await invoke<boolean>("is_config_complete");
      return isComplete;
    } catch (error) {
      return false;
    }
  }

  /**
   * Validate configuration and get missing fields
   */
  async validateConfig(): Promise<string[]> {
    try {
      const missingFields = await invoke<string[]>("validate_peeksy_config");
      return missingFields;
    } catch (error) {
      return ["Failed to validate configuration"];
    }
  }

  /**
   * Reset configuration to defaults
   */
  async resetToDefaults(): Promise<PeeksyConfig> {
    try {
      const defaultConfig = await invoke<PeeksyConfig>("reset_peeksy_config");
      return defaultConfig;
    } catch (error) {
      throw new Error("Failed to reset configuration");
    }
  }

  /**
   * Get configuration status for debugging
   */
  async getConfigStatus(): Promise<{
    config: PeeksyConfig;
    missingFields: string[];
    isComplete: boolean;
  }> {
    try {
      const [config, missingFields, isComplete] = await Promise.all([
        this.getConfig(),
        this.validateConfig(),
        this.isConfigComplete(),
      ]);

      return {
        config,
        missingFields,
        isComplete,
      };
    } catch (error) {
      throw new Error("Failed to get configuration status");
    }
  }

  /**
   * Check if API key is set
   */
  async hasApiKey(): Promise<boolean> {
    try {
      const config = await this.getConfig();
      return config.openai_api_key.trim().length > 0;
    } catch (error) {
      return false;
    }
  }

  /**
   * Check if configuration has been modified recently (within last 24 hours)
   */
  async isRecentlyUpdated(): Promise<boolean> {
    try {
      const config = await this.getConfig();
      const dayInSeconds = 24 * 60 * 60;
      const currentTime = Math.floor(Date.now() / 1000);
      return currentTime - config.updated_at < dayInSeconds;
    } catch (error) {
      return false;
    }
  }

  /**
   * Get configuration age in human readable format
   */
  async getConfigAge(): Promise<string> {
    try {
      const config = await this.getConfig();
      const ageInSeconds = Math.floor(Date.now() / 1000) - config.updated_at;

      if (ageInSeconds < 60) {
        return "Just now";
      } else if (ageInSeconds < 3600) {
        const minutes = Math.floor(ageInSeconds / 60);
        return `${minutes} minute${minutes !== 1 ? "s" : ""} ago`;
      } else if (ageInSeconds < 86400) {
        const hours = Math.floor(ageInSeconds / 3600);
        return `${hours} hour${hours !== 1 ? "s" : ""} ago`;
      } else {
        const days = Math.floor(ageInSeconds / 86400);
        return `${days} day${days !== 1 ? "s" : ""} ago`;
      }
    } catch (error) {
      return "Unknown";
    }
  }

  /**
   * Export configuration as JSON string
   */
  async exportConfig(): Promise<string> {
    try {
      const config = await this.getConfig();
      return JSON.stringify(config, null, 2);
    } catch (error) {
      throw new Error("Failed to export configuration");
    }
  }

  /**
   * Import configuration from JSON string
   */
  async importConfig(configJson: string): Promise<PeeksyConfig> {
    try {
      const config: PeeksyConfig = JSON.parse(configJson);

      // Validate required fields exist
      if (
        !config.openai_api_key &&
        !config.peeksy_prompt &&
        !config.openai_model
      ) {
        throw new Error("Invalid configuration format");
      }

      await this.saveConfig(config);
      return config;
    } catch (error) {
      throw new Error(
        "Failed to import configuration: " + (error as Error).message
      );
    }
  }
}

// Export a singleton instance
export const configManager = new ConfigManager();
