---
kind: Package
id: package:chakra-ui
name: Chakra UI Patterns
version: "3"
purpose: Document Chakra UI patterns — component library, styling props, theme customization, responsive design, and accessibility for building React applications with a design system.
problem_solved: Provides a comprehensive, accessible React component library with a design-token-based theming system, responsive style props, and built-in dark mode that enables rapid UI development without writing custom CSS or managing accessibility edge cases.
install: npm install @chakra-ui/react @emotion/react
dependencies:
  - concept:react
  - concept:design-systems
  - concept:accessibility
concepts:
  - name: ChakraProvider
    id: concept:chakra/provider
    description: "The root component that wraps your app with theme context, color mode manager, CSS reset, and global styles. ChakraProvider({ theme, resetCSS, colorModeManager }) must be the outermost wrapper. Provider enables all Chakra hooks and components to access theme tokens."
  - name: Style Props
    id: concept:chakra/style-props
    description: "Components accept CSS properties as props: <Box bg='brand.500' p={4} borderRadius='md' shadow='lg' />. Style props map to theme tokens (colors, spacing, radii, shadows, fonts) or literal values. Responsive values via array: p={[2, 4, 6]} or object: p={{ base: 2, md: 4 }}."
  - name: Theme Tokens
    id: concept:chakra/theme-tokens
    description: "Design tokens organized in extendTheme(): colors (semantic tokens), fonts, fontSizes, fontWeights, lineHeights, letterSpacings, space (spacing scale 1-64), radii, shadows, sizes, breakpoints, zIndices, borders, and transitions. Tokens are referenced by dot-notation in style props."
  - name: Color Mode
    id: concept:chakra/color-mode
    description: "Built-in dark mode with useColorMode() returning { colorMode, toggleColorMode, setColorMode }. ColorModeProvider automatically persists preference to localStorage. Components use semantic color tokens (colorScheme, _light, _dark) that change with the mode."
  - name: Responsive Styles
    id: concept:chakra/responsive
    description: "Responsive values via array or object syntax: <Box w={['100%', '50%', '25%']} /> or <Box w={{ base: '100%', md: '50%', lg: '25%' }} />. Breakpoints default to sm: 30em, md: 48em, lg: 62em, xl: 80em, 2xl: 96em. Customize in extendTheme.breakpoints."
  - name: Component Variants
    id: concept:chakra/variants
    description: "Components have predefined variants (solid, outline, ghost, subtle) and sizes (sm, md, lg). Customize via component style config in the theme. The componentStyleConfig interface defines baseStyle, sizes, variants, and defaultProps for each component."
  - name: Layout Components
    id: concept:chakra/layout
    description: "Box (div), Flex (flexbox), Grid (CSS Grid), Stack (flex with gap), HStack/VStack, Container (centered max-width), SimpleGrid (auto-fill columns), Wrap (flex wrap), Center (centering), AspectRatio (ratio-constrained container). These replace semantic divs with props-driven layout."
  - name: Forms & Inputs
    id: concept:chakra/forms
    description: "Input, Textarea, Select, NumberInput, PinInput, Switch, Checkbox, Radio, Slider, RangeSlider. FormControl wraps inputs with Label, HelpText, and ErrorMessage. useFormControl hook for custom input implementations. Form validation integrates with React Hook Form."
  - name: Disclosure Components
    id: concept:chakra/disclosure
    description: "Tabs, Accordion, Drawer, Modal, Popover, Tooltip, Menu. All handle keyboard navigation, focus management, and ARIA attributes automatically. Modal and Drawer are portal-rendered with focus trapping. Tabs support controlled and keyboard-activation modes."
  - name: Feedback Components
    id: concept:chakra/feedback
    description: "Alert (info, success, warning, error variants with icon), Toast (useToast hook, auto-dismiss, positioning), Progress (linear and circular with animations), Spinner, Skeleton (loading placeholders), Stat (metrics display). Alert and Toast use semantic color tokens."
  - name: Overlay Components
    id: concept:chakra/overlay
    description: "Modal (dialog with multiple sizes and scroll behavior), Drawer (slide-in panel from any edge), Popover (floating content with arrow), Tooltip (text on hover), Menu (dropdown). All handle positioning, collision detection, and portaling to document.body."
apis:
  - name: extendTheme()
    id: api:chakra/extend-theme
    signature: "extendTheme(baseTheme: Theme, overrides?: { colors, fonts, components, styles, config, semanticTokens }): Theme"
    returns: A merged theme object.
    description: "Extends the default Chakra theme with custom colors, typography, breakpoints, and component styles. Colors can use semantic tokens that respond to color mode. config: { initialColorMode: 'dark', useSystemColorMode: true }."
  - name: useColorMode()
    id: api:chakra/use-color-mode
    signature: "const { colorMode, toggleColorMode, setColorMode } = useColorMode() -> { colorMode: 'light' | 'dark', toggleColorMode: () => void, setColorMode: (mode) => void }"
    returns: Color mode control object.
    description: "Hook to read and toggle the current color mode. toggleColorMode switches between light and dark. setColorMode('light') forces a specific mode. The value is synced with localStorage via the ColorModeManager."
  - name: useToast()
    id: api:chakra/use-toast
    signature: "const toast = useToast(options?: { position, duration, isClosable, containerStyle }); toast({ title, description, status: 'success' | 'error' | 'info' | 'warning', duration: 5000 })"
    returns: A toast function.
    description: "Hook that returns a function to show toast notifications. Each toast call creates a dismissible notification. Auto-closes after duration ms. position (top-right default), isClosable adds an X button. id prevents duplicate toasts."
  - name: useBreakpointValue()
    id: api:chakra/use-breakpoint-value
    signature: "useBreakpointValue<T>(values: T[] | Record<string, T>, opts?: { fallback? }): T"
    returns: The value for the current breakpoint.
    description: "Returns a value based on the current viewport breakpoint. Accepts array [base, sm, md, lg, xl] or object { base, md, lg }. Useful for props that cannot use responsive arrays (like conditional rendering). Updates reactively on resize."
  - name: useDisclosure()
    id: api:chakra/use-disclosure
    signature: "const { isOpen, onOpen, onClose, onToggle } = useDisclosure(props?: { defaultIsOpen, id })"
    returns: Disclosure state and handlers.
    description: "Hook for managing open/close state for modals, drawers, popovers, and menus. Provides controlled or uncontrolled open state. isOpen is a boolean. Handlers can be passed directly to disclosure components' props (isOpen, onClose)."
sections:
  - title: Theme Configuration and Components
    id: section:chakra/theme
    content: |
      Create a custom theme with semantic tokens and component variants:

      ```typescript
      import { extendTheme } from '@chakra-ui/react';

      export const theme = extendTheme({
          config: { initialColorMode: 'light', useSystemColorMode: true },
          colors: {
              brand: { 50: '#eef2ff', 100: '#e0e7ff', 500: '#6366f1', 600: '#4f46e5', 700: '#4338ca' },
          },
          semanticTokens: {
              colors: {
                  'bg-surface': { default: 'white', _dark: 'gray.800' },
                  'bg-muted': { default: 'gray.50', _dark: 'gray.700' },
                  'text-primary': { default: 'gray.900', _dark: 'whiteAlpha.900' },
                  'text-muted': { default: 'gray.600', _dark: 'whiteAlpha.600' },
              },
          },
          components: {
              Button: {
                  baseStyle: { fontWeight: 'semibold', borderRadius: 'lg' },
                  sizes: { xl: { h: '56px', fontSize: 'lg', px: '32px' } },
                  variants: {
                      primary: { bg: 'brand.500', color: 'white', _hover: { bg: 'brand.600' } },
                  },
                  defaultProps: { size: 'lg', variant: 'primary' },
              },
          },
      });
      ```
  - title: Building a Form
    id: section:chakra/form
    content: |
      Accessible form with validation, loading state, and toast feedback:

      ```tsx
      import { Box, Button, FormControl, FormLabel, Input, FormErrorMessage, VStack, useToast, Select } from '@chakra-ui/react';
      import { useForm } from 'react-hook-form';

      function SignupForm() {
          const toast = useToast();
          const { register, handleSubmit, formState: { errors, isSubmitting } } = useForm();

          const onSubmit = async (data) => {
              try {
                  await fetch('/api/signup', { method: 'POST', body: JSON.stringify(data) });
                  toast({ title: 'Account created', status: 'success', duration: 3000 });
              } catch {
                  toast({ title: 'Error', description: 'Something went wrong', status: 'error' });
              }
          };

          return (
              <Box as="form" onSubmit={handleSubmit(onSubmit)} maxW="md">
                  <VStack spacing={4}>
                      <FormControl isInvalid={!!errors.email}>
                          <FormLabel>Email</FormLabel>
                          <Input type="email" {...register('email', { required: 'Email is required' })} />
                          <FormErrorMessage>{errors.email?.message}</FormErrorMessage>
                      </FormControl>
                      <FormControl isInvalid={!!errors.role}>
                          <FormLabel>Role</FormLabel>
                          <Select {...register('role', { required: true })}>
                              <option value="developer">Developer</option>
                              <option value="designer">Designer</option>
                          </Select>
                      </FormControl>
                      <Button type="submit" colorScheme="brand" isLoading={isSubmitting} w="full">
                          Sign Up
                      </Button>
                  </VStack>
              </Box>
          );
      }
      ```
---
