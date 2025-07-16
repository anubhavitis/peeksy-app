type ConfigsItemProps = {
  label: string;
  value: string;
  placeholder: string;
  disabled?: boolean;
};

const ConfigsItem = ({
  label,
  value,
  placeholder,
  disabled = false,
}: ConfigsItemProps) => {
  return (
    <div className="flex flex-col gap-2">
      <div className="flex justify-between gap-2">
        <label className="text-sm font-medium text-gray-800 dark:text-gray-300">
          {label}
        </label>
        {!disabled && (
          <button className="font-semibold text-white bg-red-500 rounded-md px-2 ">
            save
          </button>
        )}
      </div>
      <input
        type="text"
        className="border border-gray-400 dark:border-gray-700 rounded-md p-2 w-full focus:ring-2 focus:ring-blue-500 
            focus:border-transparent transition text-black dark:text-white"
        value={value}
        placeholder={placeholder}
        disabled={disabled}
      />
    </div>
  );
};

export default ConfigsItem;
