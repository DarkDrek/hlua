use AsMutLua;
use AsLua;

use Push;
use PushGuard;
use LuaRead;

macro_rules! tuple_impl {
    ($ty:ident) => (
        impl<LU, $ty> Push<LU> for ($ty,) where LU: AsMutLua, $ty: Push<LU> {
            fn push_to_lua(self, lua: LU) -> PushGuard<LU> {
                self.0.push_to_lua(lua)
            }
        }

        impl<LU, $ty> LuaRead<LU> for ($ty,) where LU: AsMutLua, $ty: LuaRead<LU> {
            fn lua_read_at_position(lua: LU, index: i32) -> Option<($ty,)> {
                LuaRead::lua_read_at_position(lua, index).map(|v| (v,))
            }
        }
    );

    ($first:ident, $($other:ident),+) => (
        #[allow(non_snake_case)]
        impl<LU, $first: for<'a> Push<&'a mut LU>, $($other: for<'a> Push<&'a mut LU>),+> Push<LU> for ($first, $($other),+)
                                                                          where LU: AsMutLua
        {
            fn push_to_lua(self, mut lua: LU) -> PushGuard<LU> {
                match self {
                    ($first, $($other),+) => {
                        let mut total = $first.push_to_lua(&mut lua).forget();

                        $(
                            total += $other.push_to_lua(&mut lua).forget();
                        )+

                        PushGuard { lua: lua, size: total }
                    }
                }
            }
        }

        // TODO: what if T or U are also tuples? indices won't match
        #[allow(unused_assignments)]
        #[allow(non_snake_case)]
        impl<LU, $first: for<'a> LuaRead<&'a mut LU>, $($other: for<'a> LuaRead<&'a mut LU>),+> LuaRead<LU> for ($first, $($other),+) where LU: AsLua {
            fn lua_read_at_position(mut lua: LU, index: i32) -> Option<($first, $($other),+)> {
                let mut i = index;

                let $first: Option<$first> = LuaRead::lua_read_at_position(&mut lua, i);
                i += 1;

                $(
                    let $other: Option<$other> = LuaRead::lua_read_at_position(&mut lua, i);
                    i += 1;
                )+

                if $first.is_none() {
                    return None;
                }

                if $($other.is_none())||+ {
                    return None;
                }

                Some(($first.unwrap(), $($other.unwrap()),+))

            }
        }

        tuple_impl!($($other),+);
    );
}

tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
