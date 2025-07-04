import './button.css';

export interface ButtonProps {
  primary?: boolean;
  backgroundColor?: string;
  size?: 'small' | 'medium' | 'large';
  label: string;
  onClick?: () => void;
  borderRadius?: string | number;
}

export const Button = ({
    primary = false,
    size = 'medium',
    backgroundColor,
    label,
    borderRadius,
  ...props
}: ButtonProps) => {
  const mode = primary ? 'storybook-button--primary' : 'storybook-button--secondary';
  return (
    <button
      type="button"
      className={['storybook-button', `storybook-button--${size}`, mode].join(' ')}
      style={{
        backgroundColor,
        borderRadius: borderRadius ? borderRadius : '4px',
      }}
      onClick={props.onClick}
      {...props}
    >
      {label}
    </button>
  );
};
