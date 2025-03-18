
import { CoreLayoutProps, useResourceDefinitions, useResourceContext } from "react-admin";
import { NavigationBar } from "./NavigationBar";
import { UserMenu } from "./UserMenu";
import { ModeToggle } from "@/components/mode-toggle"
import { AccordionDemo } from "@/components/sink/accordion-demo"
import { AlertDemo } from "@/components/sink/alert-demo"
import { AlertDialogDemo } from "@/components/sink/alert-dialog-demo"
import { AppSidebar } from "@/components/sink/app-sidebar"
import { AspectRatioDemo } from "@/components/sink/aspect-ratio-demo"
import { AvatarDemo } from "@/components/sink/avatar-demo"
import { BadgeDemo } from "@/components/sink/badge-demo"
import { BadgeDestructive } from "@/components/sink/badge-destructive"
import { BadgeOutline } from "@/components/sink/badge-outline"
import { BadgeSecondary } from "@/components/sink/badge-secondary"
import { BreadcrumbDemo } from "@/components/sink/breadcrumb-demo"
import { ButtonDemo } from "@/components/sink/button-demo"
import { ButtonDestructive } from "@/components/sink/button-destructive"
import { ButtonGhost } from "@/components/sink/button-ghost"
import { ButtonLink } from "@/components/sink/button-link"
import { ButtonLoading } from "@/components/sink/button-loading"
import { ButtonOutline } from "@/components/sink/button-outline"
import { ButtonSecondary } from "@/components/sink/button-secondary"
import { ButtonWithIcon } from "@/components/sink/button-with-icon"
import { CalendarDemo } from "@/components/sink/calendar-demo"
import { CardDemo } from "@/components/sink/card-demo"
import { CarouselDemo } from "@/components/sink/carousel-demo"
import { CheckboxDemo } from "@/components/sink/checkbox-demo"
import { CollapsibleDemo } from "@/components/sink/collapsible-demo"
import { ComboboxDemo } from "@/components/sink/combobox-demo"
import { CommandDemo } from "@/components/sink/command-demo"
import { ComponentWrapper } from "@/components/sink/component-wrapper"
import { ContextMenuDemo } from "@/components/sink/context-menu-demo"
import { DatePickerDemo } from "@/components/sink/date-picker-demo"
import { DialogDemo } from "@/components/sink/dialog-demo"
import { DrawerDemo } from "@/components/sink/drawer-demo"
import { DropdownMenuDemo } from "@/components/sink/dropdown-menu-demo"
import { HoverCardDemo } from "@/components/sink/hover-card-demo"
import { InputDemo } from "@/components/sink/input-demo"
import { InputOTPDemo } from "@/components/sink/input-otp-demo"
import { LabelDemo } from "@/components/sink/label-demo"
import { MenubarDemo } from "@/components/sink/menubar-demo"
import { NavigationMenuDemo } from "@/components/sink/navigation-menu-demo"
import { PaginationDemo } from "@/components/sink/pagination-demo"
import { PopoverDemo } from "@/components/sink/popover-demo"
import { ProgressDemo } from "@/components/sink/progress-demo"
import { RadioGroupDemo } from "@/components/sink/radio-group-demo"
import { ResizableHandleDemo } from "@/components/sink/resizable-handle"
import { ScrollAreaDemo } from "@/components/sink/scroll-area-demo"
import { SelectDemo } from "@/components/sink/select-demo"
import { SeparatorDemo } from "@/components/sink/separator-demo"
import { SheetDemo } from "@/components/sink/sheet-demo"
import { SkeletonDemo } from "@/components/sink/skeleton-demo"
import { SliderDemo } from "@/components/sink/slider-demo"
import { SonnerDemo } from "@/components/sink/sonner-demo"
import { SwitchDemo } from "@/components/sink/switch-demo"
import { TableDemo } from "@/components/sink/table-demo"
import { TabsDemo } from "@/components/sink/tabs-demo"
import { TextareaDemo } from "@/components/sink/textarea-demo"
import { ToastDemo } from "@/components/sink/toast-demo"
import { ToggleDemo } from "@/components/sink/toggle-demo"
import { ToggleDisabled } from "@/components/sink/toggle-disabled"
import { ToggleGroupDemo } from "@/components/sink/toggle-group-demo"
import { ToggleOutline } from "@/components/sink/toggle-outline"
import { ToggleWithText } from "@/components/sink/toggle-with-text"
import { TooltipDemo } from "@/components/sink/tooltip-demo"
import {
	Breadcrumb,
	BreadcrumbItem,
	BreadcrumbLink,
	BreadcrumbList,
	BreadcrumbPage,
	BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { Separator } from "@/components/ui/separator"
import {
	SidebarInset,
	SidebarProvider,
	SidebarTrigger,
} from "@/components/ui/sidebar"
import { Layout } from 'ra-core';

import { Toaster } from "sonner"
import { Toaster as ShadcnToaster } from "@/components/ui/toaster"

export const Layout = (props: CoreLayoutProps) => {
	const resourceDefinitions = useResourceDefinitions();

	return (
		<SidebarProvider>
			<AppSidebar/>

			<SidebarInset>
				<header className="flex h-16 shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
					<div className="flex items-center gap-2 px-4 w-full">
						<SidebarTrigger className="-ml-1" />
						<Separator orientation="vertical" className="mr-2 h-4" />
						<Breadcrumb>
							<BreadcrumbList>
								<BreadcrumbItem className="hidden md:block">
									<BreadcrumbLink href="#">
										Home
									</BreadcrumbLink>
								</BreadcrumbItem>
								<BreadcrumbSeparator className="hidden md:block" />
								<BreadcrumbItem>
									<BreadcrumbPage>Data Fetching</BreadcrumbPage>
								</BreadcrumbItem>
							</BreadcrumbList>
						</Breadcrumb>
						<div className="flex items-center gap-2 ml-auto">
							<ModeToggle />
						</div>
					</div>
				</header>
				<div className="flex flex-1 flex-col gap-4 p-4">
					{props.children}
				</div>
			</SidebarInset>
		</SidebarProvider>
	);
};
