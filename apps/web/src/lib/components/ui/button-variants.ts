import { tv, type VariantProps } from 'tailwind-variants';

export const buttonVariants = tv({
	base: 'inline-flex items-center justify-center gap-2 rounded-full font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brand-500 disabled:pointer-events-none disabled:opacity-50',
	variants: {
		variant: {
			primary: 'bg-brand-500 text-white hover:bg-brand-600',
			secondary: 'bg-surface-raised text-ink hover:bg-surface-raised/70',
			ghost: 'text-ink hover:bg-surface-raised'
		},
		size: {
			sm: 'h-9 px-4 text-sm',
			md: 'h-11 px-6 text-base',
			lg: 'h-13 px-8 text-lg'
		}
	},
	defaultVariants: { variant: 'primary', size: 'md' }
});

export type ButtonVariants = VariantProps<typeof buttonVariants>;
