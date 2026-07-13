---
kind: Package
id: package:radix-ui
name: Radix UI Primitives
version: "2"
purpose: Document Radix UI patterns — unstyled, accessible headless component primitives for building design systems with complete control over styling and behavior.
problem_solved: Provides a library of unstyled, accessible React primitives (dialog, dropdown, popover, tooltip, select, tabs, etc.) that handle ARIA attributes, keyboard navigation, focus management, and screen reader announcements, allowing developers to build custom design systems without reinventing accessibility.
install: npm install @radix-ui/react-dialog @radix-ui/react-dropdown-menu @radix-ui/react-select @radix-ui/react-tabs @radix-ui/react-tooltip
dependencies:
  - concept:react
  - concept:accessibility
  - concept:design-systems
concepts:
  - name: Headless Primitives
    id: concept:radix/headless
    description: "Radix components ship with zero CSS — they provide behavior, accessibility, and state management, but no styling. Developers apply their own classes, CSS modules, or styled-components. The separation of behavior from presentation enables any design system (Tailwind, Material, custom) as a styling layer."
  - name: Accessibility by Default
    id: concept:radix/accessibility
    description: "All primitives implement WAI-ARIA design patterns. Keyboard navigation (Arrow keys for tabs/select, Escape to dismiss), focus trapping (modals), focus restoration (return focus to trigger after close), screen reader announcements (live regions), role attributes, and aria-* properties are built-in and tested."
  - name: Dialog (Modal)
    id: concept:radix/dialog
    description: "Accessible modal dialog with focus trap, Escape dismissal, and backdrop. Dialog.Trigger, Dialog.Portal, Dialog.Overlay, Dialog.Content, Dialog.Title, Dialog.Description, Dialog.Close. The overlay receives click-outside handling. Title and Description are announced by screen readers."
  - name: Dropdown Menu
    id: concept:radix/dropdown
    description: "Context menus and dropdowns with keyboard navigation, sub-menus, separators, and disabled items. DropdownMenu.Trigger, DropdownMenu.Content, DropdownMenu.Item, DropdownMenu.CheckboxItem, DropdownMenu.RadioGroup. Sub-menu via DropdownMenu.Sub. Arrow key navigation is automatic."
  - name: Tooltip
    id: concept:radix/tooltip
    description: "Hover tooltips with configurable delay, positioning (side, align, collision detection), and animation. Tooltip.Provider wraps the app for context. Tooltip.Trigger wraps the target element. Tooltip.Content displays the tooltip text. skipDelayDuration prevents tooltip flicker between triggers."
  - name: Select
    id: concept:radix/select
    description: "Native-style select with custom rendering, search, groups, and full keyboard support. Select.Trigger displays the selected value, Select.Content positions the popup, Select.Item for options, Select.Group for category headers. Items can have indicators (checkmarks) and arbitrary React children."
  - name: Tabs
    id: concept:radix/tabs
    description: "Accessible tabbed interfaces with automatic keyboard navigation (ArrowLeft/Right), orientation support (horizontal/vertical), and programmatic activation. Tabs.List, Tabs.Trigger, Tabs.Content. The value prop controls the active tab. onValueChange callback for activation."
  - name: Slider
    id: concept:radix/slider
    description: "Range slider with single and range (two-thumb) values, min/max constraints, steps, and orientation (horizontal/vertical). Slider.Root, Slider.Track, Slider.Range (filled portion), Slider.Thumb. Accessible via Arrow keys and screen reader with aria-valuenow and aria-valuemin/max."
  - name: Collapsible
    id: concept:radix/collapsible
    description: "Expand/collapse section. Collapsible.Trigger toggles Collapsible.Content. The open/closed state is controlled or uncontrolled. Animate the content height via data-state attribute (open/closed) or use CSS animations with grid-template-rows."
  - name: Context Menu
    id: concept:radix/context-menu
    description: "Right-click context menu. ContextMenu.Trigger wraps the target, ContextMenu.Content displays on right-click. Same API as DropdownMenu but activated by contextmenu event. Supports sub-menus, separators, and item lifecycle. Prevents the browser's default context menu."
  - name: Scroll Area
    id: concept:radix/scroll-area
    description: "Custom scrollbar that works consistently across browsers and platforms. ScrollArea.Root, ScrollArea.Viewport (content), ScrollArea.Scrollbar, ScrollArea.Thumb, ScrollArea.Corner. The scrollbar can be always visible or auto-hidden. Safari, Firefox, and Chrome all get the same custom scrollbar appearance."
apis:
  - name: Dialog.Root / Dialog.Trigger / Dialog.Content
    id: api:radix/dialog-api
    signature: "<Dialog.Root open={open} onOpenChange={setOpen}><Dialog.Trigger asChild><Button>Open</Button></Dialog.Trigger><Dialog.Portal><Dialog.Overlay /><Dialog.Content><Dialog.Title /><Dialog.Description /><Dialog.Close /></Dialog.Content></Dialog.Portal></Dialog.Root>"
    returns: An accessible modal dialog.
    description: "The Dialog family of components. Root manages state. Trigger opens the dialog. Portal teleports to document.body. Overlay is the backdrop. Content is the dialog panel. Title and Description are announced by screen readers. Close button dismisses. Escape and backdrop click close by default."
  - name: DropdownMenu.Content
    id: api:radix/dropdown-menu-api
    signature: "<DropdownMenu.Content side='bottom' align='start' sideOffset={5} collisionPadding={10}>"
    returns: A positioned dropdown menu.
    description: "The content container that positions relative to the trigger. side: top/bottom/left/right. align: start/center/end. sideOffset and alignOffset for fine positioning. collisionBoundary handles edge-of-viewport detection. sticky: partial keeps the menu on the correct side during scroll."
  - name: Tooltip.Provider
    id: api:radix/tooltip-provider
    signature: "<Tooltip.Provider delayDuration={300} skipDelayDuration={300}><Tooltip.Root><Tooltip.Trigger>Hover</Tooltip.Trigger><Tooltip.Content side='top'><Tooltip.Arrow /></Tooltip.Content></Tooltip.Root></Tooltip.Provider>"
    returns: A tooltip group with shared timing.
    description: "Provider manages hover delay and skip delay across all tooltips. delayDuration: time before tooltip appears. skipDelayDuration: when moving between triggers, the delay is skipped during this window (prevents flickering). Wrap the entire app or a section."
  - name: Select.Item
    id: api:radix/select-item
    signature: "<Select.Item value='option-1' disabled={false}> <Select.ItemText>Option 1</Select.ItemText> <Select.ItemIndicator>✓</Select.ItemIndicator> </Select.Item>"
    returns: A selectable option.
    description: "An option in the Select. value must be unique across the Select. disabled prevents selection. ItemIndicator renders when the item is selected. ItemText is the visible text. Items can have arbitrary children for rich option rendering (icons, descriptions, badges)."
  - name: asChild
    id: api:radix/as-child
    signature: "<Dialog.Trigger asChild>{children: ReactElement} => clones the child and merges handlers"
    returns: A cloned element with Radix props.
    description: "Radix's composition API — renders its children as the trigger instead of a default element. The child must accept a ref and handle onClick/onKeyDown. Enables using any styled button or element as the trigger. Only one child allowed. This is the recommended pattern."
sections:
  - title: Composing a Dialog with Custom Styles
    id: section:radix/dialog-composition
    content: |
      Build a styled modal dialog using Radix primitives with Tailwind:

      ```tsx
      import * as Dialog from '@radix-ui/react-dialog';
      import { Cross2Icon } from '@radix-ui/react-icons';

      export function Modal({ children, title, trigger }: ModalProps) {
          return (
              <Dialog.Root>
                  <Dialog.Trigger asChild>{trigger}</Dialog.Trigger>
                  <Dialog.Portal>
                      <Dialog.Overlay className="fixed inset-0 bg-black/50 data-[state=open]:animate-overlayShow" />
                      <Dialog.Content className="fixed left-1/2 top-1/2 max-w-md w-full -translate-x-1/2 -translate-y-1/2 rounded-lg bg-white p-6 shadow-lg data-[state=open]:animate-contentShow">
                          <Dialog.Title className="text-lg font-semibold">{title}</Dialog.Title>
                          <Dialog.Description className="text-sm text-gray-500 mt-2">
                              {children}
                          </Dialog.Description>
                          <Dialog.Close asChild>
                              <button className="absolute top-4 right-4" aria-label="Close">
                                  <Cross2Icon />
                              </button>
                          </Dialog.Close>
                      </Dialog.Content>
                  </Dialog.Portal>
              </Dialog.Root>
          );
      }
      ```

      Animations use Radix's data-state attributes. Define keyframes in CSS:

      ```css
      @keyframes overlayShow { from { opacity: 0 } to { opacity: 1 } }
      @keyframes contentShow { from { opacity: 0; transform: translate(-50%, -48%) scale(0.96) } to { opacity: 1; transform: translate(-50%, -50%) scale(1) } }
      ```
  - title: Dropdown Menu with Icons
    id: section:radix/dropdown-example
    content: |
      A right-click context menu with icons and separators:

      ```tsx
      import * as ContextMenu from '@radix-ui/react-context-menu';
      import { ClipboardIcon, Pencil1Icon, TrashIcon } from '@radix-ui/react-icons';

      export function FileContextMenu({ onRename, onDelete, children }: Props) {
          return (
              <ContextMenu.Root>
                  <ContextMenu.Trigger asChild>{children}</ContextMenu.Trigger>
                  <ContextMenu.Portal>
                      <ContextMenu.Content className="min-w-40 rounded-md bg-white shadow-lg py-1 border">
                          <ContextMenu.Item onSelect={onRename} className="flex items-center px-3 py-2 hover:bg-gray-100 cursor-pointer">
                              <Pencil1Icon className="mr-2" /> Rename
                          </ContextMenu.Item>
                          <ContextMenu.Item className="flex items-center px-3 py-2 hover:bg-gray-100 cursor-pointer">
                              <ClipboardIcon className="mr-2" /> Copy
                          </ContextMenu.Item>
                          <ContextMenu.Separator className="h-px bg-gray-200 my-1" />
                          <ContextMenu.Item onSelect={onDelete} className="flex items-center px-3 py-2 text-red-600 hover:bg-red-50 cursor-pointer">
                              <TrashIcon className="mr-2" /> Delete
                          </ContextMenu.Item>
                      </ContextMenu.Content>
                  </ContextMenu.Portal>
              </ContextMenu.Root>
          );
      }
      ```
---
