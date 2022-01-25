pub trait InspectSome<T, F>
where
    F: Fn(&T),
{
    fn inspect_some(self, f: F) -> Self;
}

impl<T, F> InspectSome<T, F> for Option<T>
where
    F: Fn(&T),
{
    fn inspect_some(self, f: F) -> Self {
        if let Some(t) = self.as_ref() {
            f(t);
        };

        self
    }
}

pub trait InspectNone<F>
where
    F: Fn(),
{
    fn inspect_none(self, f: F) -> Self;
}

impl<T, F> InspectNone<F> for Option<T>
where
    F: Fn(),
{
    fn inspect_none(self, f: F) -> Self {
        if self.is_none() {
            f();
        }

        self
    }
}

pub trait InspectOk<T, F>
where
    F: Fn(&T),
{
    fn inspect_ok(self, f: F) -> Self;
}

impl<T, E, F> InspectOk<T, F> for Result<T, E>
where
    F: Fn(&T),
{
    fn inspect_ok(self, f: F) -> Self {
        if let Ok(t) = self.as_ref() {
            f(t)
        }

        self
    }
}

pub trait InspectErr<E, F>
where
    F: Fn(&E),
{
    fn inspect_err(self, f: F) -> Self;
}

impl<T, E, F> InspectErr<E, F> for Result<T, E>
where
    F: Fn(&E),
{
    fn inspect_err(self, f: F) -> Self {
        if let Err(e) = self.as_ref() {
            f(e);
        }

        self
    }
}

pub trait Inspect<F>
where
    F: Fn(&Self),
{
    fn inspect(self, f: F) -> Self;
}

impl<T, F> Inspect<F> for T
where
    F: Fn(&Self),
{
    fn inspect(self, f: F) -> Self {
        f(&self);

        self
    }
}
