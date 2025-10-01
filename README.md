# BoundedExp

Sample from an exponential distribution with a maximum value.

I needed a way to sample decay times for a long-lived isotope (half life of `half_life`). Rather than sampling a bunch of times and rejecting the values that exceeded the `measurement_time`, I
$$ p(t) = \frac{\lambda \mathrm{e}^{-\lambda t}}{1 - \mathrm{e}^{-\lambda \tau}},$$
where $\tau$ is the measurement time and $\lambda=\frac{\mathrm{ln}2}{\mathrm{half\_life}}$.
