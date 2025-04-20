use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AnimationType {
    /// Animation for typing effect in messages
    Typing,
    /// Animation for message fade-in effect
    FadeIn,
    /// Animation for message slide-in effect
    SlideIn,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub start_time: Instant,
    pub duration: Duration,
    pub is_complete: bool,
}

impl Animation {
    pub fn new(animation_type: AnimationType, duration: Duration) -> Self {
        Self {
            animation_type,
            start_time: Instant::now(),
            duration,
            is_complete: false,
        }
    }

    pub fn update(&mut self) {
        if self.is_complete {
            return;
        }

        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            self.is_complete = true;
        }
    }

    pub fn progress(&self) -> f32 {
        if self.is_complete {
            return 1.0;
        }

        let elapsed = self.start_time.elapsed();
        let raw_progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        
        match self.animation_type {
            AnimationType::Typing => raw_progress,
            AnimationType::FadeIn => raw_progress,
            AnimationType::SlideIn => {
                // Ease-out cubic for smooth slide-in
                1.0 - (1.0 - raw_progress).powf(3.0)
            }
        }
    }
} 