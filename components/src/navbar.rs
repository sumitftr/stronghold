use dioxus::prelude::*;
use primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        navbar::Navbar {
            class: "flex box-border p-1 border-none rounded-lg gap-1",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: "relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger {
            class: "flex flex-row items-center justify-center px-3 py-2 border-none rounded bg-transparent text-gray-600 dark:text-gray-400 cursor-pointer transition-colors duration-100 ease-out hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-100 focus-visible:bg-gray-100 dark:focus-visible:bg-gray-700 focus-visible:text-gray-900 dark:focus-visible:text-gray-100 focus-visible:outline-none data-[disabled=true]:text-gray-400 dark:data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
            attributes: props.attributes,
            {props.children}
            svg {
                class: "w-5 h-5 fill-none stroke-gray-600 dark:stroke-gray-400 stroke-2 transition-transform duration-150 ease-in-out [.navbar-nav[data-state='open']_&]:rotate-180",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: "absolute z-[1000] top-full left-0 min-w-[200px] p-1 rounded-lg mt-2 bg-white dark:bg-gray-800 shadow-[inset_0_0_0_1px_rgb(229_231_235)] dark:shadow-[inset_0_0_0_1px_rgb(55_65_81)] opacity-0 pointer-events-none origin-top will-change-[transform,opacity] data-[state=closed]:transition-[opacity_150ms_ease-in,transform_150ms_ease-in] data-[state=closed]:data-[open-menu-direction=start]:-translate-x-full data-[state=closed]:data-[open-menu-direction=start]:scale-[0.98] data-[state=closed]:data-[open-menu-direction=end]:translate-x-full data-[state=closed]:data-[open-menu-direction=end]:scale-[0.98] data-[state=closed]:data-[open-menu-direction=closed]:translate-y-4 data-[state=closed]:data-[open-menu-direction=closed]:scale-[0.98] data-[state=open]:opacity-100 data-[state=open]:pointer-events-auto data-[state=open]:translate-x-0 data-[state=open]:translate-y-0 data-[state=open]:scale-100 data-[state=open]:transition-[opacity_200ms_ease-out,transform_200ms_cubic-bezier(0.16,1,0.3,1)] before:absolute before:-top-2 before:left-0 before:w-full before:h-2 before:content-[''] [.navbar-nav:first-child_&]:-ml-1",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            class: "block px-3 py-2 rounded text-gray-600 dark:text-gray-400 cursor-pointer text-sm no-underline hover:text-[#00c573] dark:hover:text-[#00c573] focus-visible:bg-gray-100 dark:focus-visible:bg-gray-700 focus-visible:text-gray-900 dark:focus-visible:text-gray-100 focus-visible:outline-none data-[disabled=true]:text-gray-400 dark:data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
