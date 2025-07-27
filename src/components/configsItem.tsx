import { useState, useRef, useEffect } from "react";

type ConfigsItemProps = {
  label: string;
  value: string;
  placeholder: string;
  disabled?: boolean;
  sensitive?: boolean;
  multiline?: boolean;
  onSave?: (value: string) => Promise<boolean>;
};

const ConfigsItem = ({
  label,
  value,
  placeholder,
  disabled = false,
  sensitive = false,
  multiline = false,
  onSave,
}: ConfigsItemProps) => {
  const [currentValue, setCurrentValue] = useState(value);
  const [isSaving, setIsSaving] = useState(false);
  const [showSensitive, setShowSensitive] = useState(false);
  const inputRef = useRef<HTMLInputElement | HTMLTextAreaElement>(null);

  // Update local state when prop changes from parent
  useEffect(() => {
    setCurrentValue(value);
  }, [value]);

  const hasChanges = currentValue !== value;

  const handleSave = async () => {
    if (!onSave || !hasChanges) return;

    setIsSaving(true);
    try {
      await onSave(currentValue);
    } finally {
      setIsSaving(false);
    }
  };

  const handleCancel = () => {
    setCurrentValue(value);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !multiline) {
      e.preventDefault();
      handleSave();
    } else if (e.key === "Escape") {
      handleCancel();
    }
  };

  const displayValue =
    sensitive && !showSensitive && currentValue
      ? "•".repeat(Math.min(currentValue.length, 20))
      : currentValue;

  const InputComponent = multiline ? "textarea" : "input";

  return (
    <div className="flex flex-col gap-2">
      <div className="flex justify-between items-center gap-2">
        <label className="text-sm font-medium text-gray-800 dark:text-gray-300">
          {label}
        </label>

        <div className="flex gap-2 items-center">
          {sensitive && currentValue && (
            <button
              onClick={() => setShowSensitive(!showSensitive)}
              className="text-xs text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300 px-2 py-1 rounded"
            >
              {showSensitive ? "Hide" : "Show"}
            </button>
          )}

          {!disabled && hasChanges && (
            <div className="flex gap-1">
              <button
                onClick={handleCancel}
                className="font-semibold text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 rounded-md px-2 py-1 text-sm transition"
                disabled={isSaving}
              >
                Cancel
              </button>
              <button
                onClick={handleSave}
                disabled={isSaving}
                className="font-semibold text-white bg-green-500 hover:bg-green-600 disabled:bg-gray-400 disabled:cursor-not-allowed rounded-md px-3 py-1 text-sm transition"
              >
                {isSaving ? "Saving..." : "Save"}
              </button>
            </div>
          )}
        </div>
      </div>

      <InputComponent
        ref={inputRef as any}
        type={sensitive && !showSensitive ? "password" : "text"}
        className={`border border-gray-400 dark:border-gray-700 rounded-md p-2 w-full focus:ring-2 focus:ring-blue-500 
            focus:border-transparent transition text-black dark:text-white dark:bg-gray-800 
            ${disabled ? "bg-gray-100 dark:bg-gray-700 cursor-not-allowed" : ""}
            ${multiline ? "min-h-[100px] resize-y" : ""}`}
        value={displayValue}
        placeholder={placeholder}
        disabled={disabled}
        onChange={(e) => setCurrentValue(e.target.value)}
        onKeyDown={handleKeyDown}
        rows={multiline ? 4 : undefined}
      />

      {hasChanges && (
        <div className="text-xs text-amber-600 dark:text-amber-400">
          You have unsaved changes. Press Enter to save or Escape to cancel.
        </div>
      )}
    </div>
  );
};

export default ConfigsItem;
